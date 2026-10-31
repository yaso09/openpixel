use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

mod update;
pub use update::update_binaries;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub width: usize,
    pub height: usize,
    #[serde(default = "default_true", rename = "loop")]
    pub looping: bool,
    pub layers: Vec<Layer>,
    pub frames: Vec<Frame>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layer {
    pub id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    #[serde(rename = "durationMs")]
    pub duration_ms: u32,
    pub layers: Vec<FrameLayer>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrameLayer {
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub pixels: Vec<Vec<Option<String>>>,
}
fn default_true() -> bool {
    true
}
fn default_opacity() -> f32 {
    1.0
}

pub fn read_doc(path: &str) -> Result<Document> {
    let text = fs::read_to_string(path).with_context(|| format!("could not read {path}"))?;
    let doc: Document =
        serde_json::from_str(&text).with_context(|| format!("invalid JSON asset in {path}"))?;
    validate(&doc)?;
    Ok(doc)
}
pub fn validate(doc: &Document) -> Result<()> {
    if doc.version != 1 {
        bail!("version must be 1");
    }
    if !(1..=4096).contains(&doc.width) || !(1..=2304).contains(&doc.height) {
        bail!("width must be from 1 to 4096 and height from 1 to 2304");
    }
    if doc.width as usize * doc.height as usize > 9_437_184 {
        bail!("canvas must not exceed 9,437,184 pixels");
    }
    if doc.layers.is_empty() || doc.frames.is_empty() {
        bail!("at least one layer and frame are required");
    }
    let mut ids = std::collections::HashSet::new();
    for layer in &doc.layers {
        if layer.id.is_empty()
            || !layer
                .id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            bail!(
                "invalid layer id '{}': use letters, digits, _ or -",
                layer.id
            );
        }
        if !ids.insert(layer.id.as_str()) {
            bail!("duplicate layer id '{}'", layer.id);
        }
        if layer.name.trim().is_empty() {
            bail!("layer '{}' must have a name", layer.id);
        }
        if !layer.opacity.is_finite() || !(0.0..=1.0).contains(&layer.opacity) {
            bail!("layer '{}' opacity must be from 0 to 1", layer.id);
        }
    }
    for (frame_index, frame) in doc.frames.iter().enumerate() {
        if !(1..=60000).contains(&frame.duration_ms) {
            bail!("frame {frame_index} durationMs must be from 1 to 60000");
        }
        let mut seen = std::collections::HashSet::new();
        for entry in &frame.layers {
            if !ids.contains(entry.layer_id.as_str()) {
                bail!(
                    "frame {frame_index} references unknown layer '{}'",
                    entry.layer_id
                );
            }
            if !seen.insert(entry.layer_id.as_str()) {
                bail!("frame {frame_index} repeats layer '{}'", entry.layer_id);
            }
            if entry.pixels.len() != doc.height {
                bail!(
                    "frame {frame_index}, layer '{}': expected {} rows",
                    entry.layer_id,
                    doc.height
                );
            }
            for (y, row) in entry.pixels.iter().enumerate() {
                if row.len() != doc.width {
                    bail!(
                        "frame {frame_index}, layer '{}' row {y}: expected {} columns",
                        entry.layer_id,
                        doc.width
                    );
                }
                for (x, color) in row.iter().enumerate() {
                    if let Some(color) = color {
                        if !valid_color(color) {
                            bail!("invalid color at frame {frame_index}, layer '{}', ({x},{y}): {color}", entry.layer_id);
                        }
                    }
                }
            }
        }
        if seen.len() != ids.len() {
            bail!("frame {frame_index} must contain every layer exactly once");
        }
    }
    Ok(())
}
pub fn valid_color(color: &str) -> bool {
    (color.len() == 7 || color.len() == 9)
        && color.starts_with('#')
        && color[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
}
pub fn write_doc(path: &str, doc: &Document) -> Result<()> {
    validate(doc)?;
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let json = serde_json::to_string_pretty(doc)?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("could not write {path}"))
}
pub fn render_svg(
    doc: &Document,
    frame_index: usize,
    scale: usize,
    background: Option<&str>,
    title: Option<&str>,
) -> Result<String> {
    validate(doc)?;
    if frame_index >= doc.frames.len() {
        bail!("frame index must be between 0 and {}", doc.frames.len() - 1);
    }
    if scale == 0 {
        bail!("scale must be greater than zero");
    }
    if let Some(bg) = background {
        if !valid_color(bg) {
            bail!("background must be #RRGGBB or #RRGGBBAA");
        }
    }
    let title = title.or(doc.name.as_deref()).unwrap_or("Pixel art");
    let frame = &doc.frames[frame_index];
    let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" shape-rendering=\"crispEdges\" role=\"img\" aria-label=\"{}\">", doc.width, doc.height, doc.width * scale, doc.height * scale, escape_xml(title));
    svg.push_str(&format!("<title>{}</title>", escape_xml(title)));
    if let Some(bg) = background {
        svg.push_str(&format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>",
            doc.width, doc.height, bg
        ));
    }
    for layer in &doc.layers {
        if !layer.visible {
            continue;
        }
        let entry = frame
            .layers
            .iter()
            .find(|item| item.layer_id == layer.id)
            .context("validated layer missing")?;
        svg.push_str(&format!(
            "<g data-layer=\"{}\" opacity=\"{}\">",
            escape_xml(&layer.id),
            layer.opacity
        ));
        for (y, row) in entry.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                if let Some(color) = pixel {
                    svg.push_str(&format!(
                        "<rect x=\"{x}\" y=\"{y}\" width=\"1\" height=\"1\" fill=\"{color}\"/>"
                    ));
                }
            }
        }
        svg.push_str("</g>");
    }
    svg.push_str("</svg>");
    Ok(svg)
}

/// Encode the document's timed frames as an H.264 MP4 using ffmpeg.
pub fn export_mp4(
    doc: &Document,
    output: &Path,
    fps: u32,
    scale: usize,
) -> Result<(usize, usize, usize)> {
    validate(doc)?;
    if !(1..=60).contains(&fps) {
        bail!("fps must be from 1 to 60");
    }
    if !(1..=16).contains(&scale) {
        bail!("scale must be from 1 to 16");
    }
    if output
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        != Some("mp4".into())
    {
        bail!("output file must use the .mp4 extension");
    }
    let width = doc
        .width
        .checked_mul(scale)
        .context("scaled video width overflows")?;
    let height = doc
        .height
        .checked_mul(scale)
        .context("scaled video height overflows")?;
    if width
        .checked_mul(height)
        .context("scaled video dimensions overflow")?
        > 16_777_216
    {
        bail!("scaled video must not exceed 16,777,216 pixels per frame");
    }
    let total_ms: u64 = doc
        .frames
        .iter()
        .map(|frame| frame.duration_ms as u64)
        .sum();
    if total_ms > 600_000 {
        bail!("video duration must not exceed 10 minutes");
    }
    let frame_count = ((total_ms * fps as u64 + 500) / 1000).max(1) as usize;
    let visible_layers: Vec<_> = doc
        .layers
        .iter()
        .filter(|layer| layer.visible)
        .map(|layer| {
            let entry = doc.frames[0]
                .layers
                .iter()
                .find(|entry| entry.layer_id == layer.id)
                .expect("validated layer");
            (layer.opacity, entry)
        })
        .collect();
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut child = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-n", "-f", "rawvideo", "-pixel_format", "rgb24"])
        .arg("-video_size").arg(format!("{width}x{height}"))
        .arg("-framerate").arg(fps.to_string())
        .args(["-i", "pipe:0", "-an", "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2", "-c:v", "libx264", "-preset", "medium", "-crf", "18", "-pix_fmt", "yuv420p", "-movflags", "+faststart"])
        .arg(output)
        .stdin(Stdio::piped()).stderr(Stdio::piped())
        .spawn().context("could not start ffmpeg; install ffmpeg with libx264 support and make it available on PATH")?;

    let stream_result = (|| -> Result<()> {
        let input = child
            .stdin
            .as_mut()
            .context("ffmpeg stdin is unavailable")?;
        let mut elapsed = 0_u64;
        let frame_ends: Vec<u64> = doc
            .frames
            .iter()
            .map(|frame| {
                elapsed += frame.duration_ms as u64;
                elapsed
            })
            .collect();
        let mut repeats = vec![0_usize; doc.frames.len()];
        for output_index in 0..frame_count {
            let time_ms = output_index as u64 * 1000 / fps as u64;
            let source_index = frame_ends
                .iter()
                .position(|end| time_ms < *end)
                .unwrap_or(doc.frames.len() - 1);
            repeats[source_index] += 1;
        }
        for (source_index, repeat_count) in repeats.into_iter().enumerate() {
            if repeat_count == 0 {
                continue;
            }
            let frame = write_rgb_frame(doc, source_index, scale, &visible_layers)?;
            for _ in 0..repeat_count {
                input
                    .write_all(&frame)
                    .context("could not stream pixel frame to ffmpeg")?;
            }
        }
        Ok(())
    })();
    drop(child.stdin.take());
    if let Err(error) = stream_result {
        let _ = child.kill();
        let _ = child.wait();
        return Err(error);
    }
    let status = child.wait().context("could not wait for ffmpeg")?;
    if !status.success() {
        let mut stderr = String::new();
        if let Some(mut pipe) = child.stderr.take() {
            let _ = pipe.read_to_string(&mut stderr);
        }
        bail!("ffmpeg could not encode MP4: {}", stderr.trim());
    }
    Ok((width + width % 2, height + height % 2, frame_count))
}

fn write_rgb_frame(
    doc: &Document,
    frame_index: usize,
    scale: usize,
    visible_layers: &[(f32, &FrameLayer)],
) -> Result<Vec<u8>> {
    let output_width = doc.width * scale;
    let mut expanded_row = Vec::with_capacity(output_width * 3);
    let mut output = Vec::with_capacity(output_width * doc.height * scale * 3);
    let frame_layers: Vec<(f32, &FrameLayer)> = visible_layers
        .iter()
        .map(|(opacity, base_entry)| {
            let entry = doc.frames[frame_index]
                .layers
                .iter()
                .find(|entry| entry.layer_id == base_entry.layer_id)
                .expect("validated frame layer");
            (*opacity, entry)
        })
        .collect();
    for source_y in 0..doc.height {
        expanded_row.clear();
        for x in 0..doc.width {
            let mut rgb = [0.0_f32; 3];
            for (opacity, entry) in &frame_layers {
                if let Some(color) = &entry.pixels[source_y][x] {
                    let raw = &color.as_bytes()[1..];
                    let channel = |index: usize| {
                        u8::from_str_radix(
                            std::str::from_utf8(&raw[index..index + 2]).expect("ASCII hex"),
                            16,
                        )
                        .expect("validated hex") as f32
                    };
                    let alpha = (if raw.len() == 8 {
                        channel(6) / 255.0
                    } else {
                        1.0
                    }) * *opacity;
                    for index in 0..3 {
                        rgb[index] = channel(index * 2) * alpha + rgb[index] * (1.0 - alpha);
                    }
                }
            }
            let pixel = [
                rgb[0].round() as u8,
                rgb[1].round() as u8,
                rgb[2].round() as u8,
            ];
            for _ in 0..scale {
                expanded_row.extend_from_slice(&pixel);
            }
        }
        for _ in 0..scale {
            output.extend_from_slice(&expanded_row);
        }
    }
    Ok(output)
}
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
