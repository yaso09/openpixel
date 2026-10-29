use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    if !(1..=2048).contains(&doc.width) || !(1..=1152).contains(&doc.height) {
        bail!("width must be from 1 to 2048 and height from 1 to 1152");
    }
    if doc.width as usize * doc.height as usize > 2_359_296 {
        bail!("canvas must not exceed 2,359,296 pixels");
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
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
