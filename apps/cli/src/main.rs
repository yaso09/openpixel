use anyhow::{bail, Context, Result};
use openpixel_cli::{
    read_doc, render_svg, valid_color, validate, write_doc, Document, Frame, FrameLayer, Layer,
};
use std::{collections::BTreeMap, env, fs, path::Path, process};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        process::exit(1);
    }
}
fn run() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "help" | "--help" | "-h") {
        print_help();
        return Ok(());
    }
    let command = args.remove(0);
    match command.as_str() {
        "validate" => {
            let path = required_pos(&mut args, "input file")?;
            reject_extra(&args)?;
            let doc = read_doc(&path)?;
            validate(&doc)?;
            println!(
                "Valid OpenPixel v1 asset: {}x{}, {} layer(s), {} frame(s)",
                doc.width,
                doc.height,
                doc.layers.len(),
                doc.frames.len()
            );
        }
        "inspect" => {
            let path = required_pos(&mut args, "input file")?;
            reject_extra(&args)?;
            inspect(&read_doc(&path)?)?
        }
        "render" => render_command(args)?,
        "render-frames" => render_frames_command(args)?,
        "new" => new_command(args)?,
        "set-pixel" => set_pixel_command(args)?,
        "set-layer" => set_layer_command(args)?,
        "add-frame" => add_frame_command(args)?,
        "set-frame" => set_frame_command(args)?,
        "set-loop" => set_loop_command(args)?,
        other => bail!("unknown command '{other}'. Run `openpixel help` for usage."),
    }
    Ok(())
}
fn print_help() {
    println!(
        "OpenPixel — pixel art asset tools\n\
Usage: openpixel <command> [options]\n\
Commands:\n\
  new <file> --width N --height N [--name NAME] [--layers id[:name],...]\n\
  validate <file>                         Validate a v1 JSON asset\n\
  inspect <file>                          Show asset, frame, layer, and palette info\n\
  render <file> --output FILE [--frame N] [--scale N] [--background #RRGGBB] [--title TEXT]\n\
  render-frames <file> --out-dir DIR [--scale N] [--title TEXT]  Export every frame as SVG\n\
  set-pixel <file> --output FILE --frame N --layer ID --x N --y N --color #RRGGBB|null\n\
  set-layer <file> --output FILE --layer ID [--name NAME] [--visible true|false] [--opacity 0..1]\n\
  add-frame <file> --output FILE [--source-frame N] [--duration-ms N]\n\
  set-frame <file> --output FILE --frame N --duration-ms N\n\
  set-loop <file> --output FILE --enabled true|false\n\
  help                                    Show this help\n\
\nAll pixel coordinates are zero-based. Layer list order defines back-to-front drawing order."
    );
}
fn required_pos(args: &mut Vec<String>, label: &str) -> Result<String> {
    if args.is_empty() {
        bail!("missing {label}");
    }
    Ok(args.remove(0))
}
fn option(args: &mut Vec<String>, key: &str, default: Option<String>) -> Result<Option<String>> {
    if let Some(i) = args.iter().position(|a| a == key) {
        if i + 1 >= args.len() {
            bail!("{key} requires a value");
        }
        args.remove(i);
        Ok(Some(args.remove(i)))
    } else {
        Ok(default)
    }
}
fn number<T: std::str::FromStr>(value: Option<String>, label: &str, default: Option<T>) -> Result<T>
where
    T::Err: std::fmt::Display,
{
    match value {
        Some(s) => s
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid {label} '{s}': {e}")),
        None => default.ok_or_else(|| anyhow::anyhow!("missing {label}")),
    }
}
fn inspect(doc: &Document) -> Result<()> {
    let mut palette = BTreeMap::<String, usize>::new();
    for frame in &doc.frames {
        for layer in &frame.layers {
            for row in &layer.pixels {
                for pixel in row {
                    if let Some(color) = pixel {
                        *palette.entry(color.clone()).or_default() += 1;
                    }
                }
            }
        }
    }
    println!(
        "{} — {}x{}",
        doc.name.as_deref().unwrap_or("Untitled"),
        doc.width,
        doc.height
    );
    println!(
        "{} layer(s), {} frame(s), loop: {}",
        doc.layers.len(),
        doc.frames.len(),
        doc.looping
    );
    for layer in &doc.layers {
        println!(
            "  layer {} ({}) visible={} opacity={}",
            layer.id, layer.name, layer.visible, layer.opacity
        );
    }
    for (i, frame) in doc.frames.iter().enumerate() {
        println!("  frame {i}: {} ms", frame.duration_ms);
    }
    println!("palette ({} colors):", palette.len());
    for (color, count) in palette {
        println!("  {color}: {count} pixel(s)");
    }
    Ok(())
}
fn render_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let frame = number(
        option(&mut args, "--frame", Some("0".into()))?,
        "frame index",
        Some(0usize),
    )?;
    let scale = number(
        option(&mut args, "--scale", Some("1".into()))?,
        "scale",
        Some(1usize),
    )?;
    let background = option(&mut args, "--background", None)?;
    let title = option(&mut args, "--title", None)?;
    reject_extra(&args)?;
    let doc = read_doc(&input)?;
    write_svg(
        &doc,
        frame,
        scale,
        background.as_deref(),
        title.as_deref(),
        Path::new(&output),
    )
}
fn render_frames_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let out_dir = option(&mut args, "--out-dir", None)?.context("missing --out-dir")?;
    let scale = number(
        option(&mut args, "--scale", Some("1".into()))?,
        "scale",
        Some(1usize),
    )?;
    let title = option(&mut args, "--title", None)?;
    reject_extra(&args)?;
    let doc = read_doc(&input)?;
    fs::create_dir_all(&out_dir).with_context(|| format!("could not create {out_dir}"))?;
    let digits = doc.frames.len().to_string().len().max(4);
    for i in 0..doc.frames.len() {
        let path = Path::new(&out_dir).join(format!("frame-{:0digits$}.svg", i + 1));
        write_svg(&doc, i, scale, None, title.as_deref(), &path)?;
    }
    println!("Exported {} SVG frame(s) to {out_dir}", doc.frames.len());
    Ok(())
}
fn write_svg(
    doc: &Document,
    frame_index: usize,
    scale: usize,
    background: Option<&str>,
    title: Option<&str>,
    path: &Path,
) -> Result<()> {
    let svg = render_svg(doc, frame_index, scale, background, title)?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, svg).with_context(|| format!("could not write {}", path.display()))?;
    println!("Rendered frame {} to {}", frame_index, path.display());
    Ok(())
}
fn new_command(mut args: Vec<String>) -> Result<()> {
    let output = required_pos(&mut args, "output file")?;
    let width = number(option(&mut args, "--width", None)?, "--width", None)?;
    let height = number(option(&mut args, "--height", None)?, "--height", None)?;
    let name = option(&mut args, "--name", None)?;
    let layer_arg = option(&mut args, "--layers", Some("base:Base".into()))?.unwrap();
    reject_extra(&args)?;
    let layers: Vec<Layer> = layer_arg
        .split(',')
        .map(|part| {
            let (id, label) = part.split_once(':').unwrap_or((part, part));
            Layer {
                id: id.trim().into(),
                name: label.trim().into(),
                visible: true,
                opacity: 1.0,
            }
        })
        .collect();
    let entries = layers
        .iter()
        .map(|layer| FrameLayer {
            layer_id: layer.id.clone(),
            pixels: vec![vec![None; width]; height],
        })
        .collect();
    let doc = Document {
        version: 1,
        name,
        width,
        height,
        looping: true,
        layers,
        frames: vec![Frame {
            duration_ms: 100,
            layers: entries,
        }],
    };
    validate(&doc)?;
    write_doc(&output, &doc)?;
    println!("Created {}x{} asset at {output}", width, height);
    Ok(())
}
fn set_pixel_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let frame_index: usize = number(option(&mut args, "--frame", None)?, "--frame", None)?;
    let layer_id = option(&mut args, "--layer", None)?.context("missing --layer")?;
    let x: usize = number(option(&mut args, "--x", None)?, "--x", None)?;
    let y: usize = number(option(&mut args, "--y", None)?, "--y", None)?;
    let color = option(&mut args, "--color", None)?.context("missing --color")?;
    reject_extra(&args)?;
    let mut doc = read_doc(&input)?;
    if x >= doc.width || y >= doc.height {
        bail!(
            "coordinates must be within 0..{} by 0..{}",
            doc.width,
            doc.height
        );
    }
    if frame_index >= doc.frames.len() {
        bail!("frame index is outside the asset");
    }
    let layer = doc.frames[frame_index]
        .layers
        .iter_mut()
        .find(|entry| entry.layer_id == layer_id)
        .context("layer id not found")?;
    layer.pixels[y][x] = if color == "null" {
        None
    } else {
        if !valid_color(&color) {
            bail!("color must be #RRGGBB, #RRGGBBAA, or null");
        }
        Some(color)
    };
    write_doc(&output, &doc)?;
    println!("Updated pixel ({x},{y}) in frame {frame_index}, layer {layer_id}");
    Ok(())
}
fn set_layer_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let layer_id = option(&mut args, "--layer", None)?.context("missing --layer")?;
    let name = option(&mut args, "--name", None)?;
    let visible = option(&mut args, "--visible", None)?
        .map(|s| parse_bool(&s))
        .transpose()?;
    let opacity = option(&mut args, "--opacity", None)?
        .map(|s| s.parse::<f32>().context("invalid --opacity"))
        .transpose()?;
    reject_extra(&args)?;
    if name.is_none() && visible.is_none() && opacity.is_none() {
        bail!("provide at least one of --name, --visible, or --opacity");
    }
    let mut doc = read_doc(&input)?;
    let layer = doc
        .layers
        .iter_mut()
        .find(|layer| layer.id == layer_id)
        .context("layer id not found")?;
    if let Some(name) = name {
        layer.name = name;
    }
    if let Some(visible) = visible {
        layer.visible = visible;
    }
    if let Some(opacity) = opacity {
        layer.opacity = opacity;
    }
    write_doc(&output, &doc)?;
    println!("Updated layer {layer_id}");
    Ok(())
}
fn set_loop_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let enabled = parse_bool(&option(&mut args, "--enabled", None)?.context("missing --enabled")?)?;
    reject_extra(&args)?;
    let mut doc = read_doc(&input)?;
    doc.looping = enabled;
    write_doc(&output, &doc)?;
    println!("Looping set to {enabled}");
    Ok(())
}
fn parse_bool(value: &str) -> Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => bail!("expected true or false, got '{value}'"),
    }
}
fn add_frame_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let source: usize = number(
        option(&mut args, "--source-frame", Some("0".into()))?,
        "source frame",
        Some(0usize),
    )?;
    let duration = number(
        option(&mut args, "--duration-ms", Some("100".into()))?,
        "duration",
        Some(100u32),
    )?;
    reject_extra(&args)?;
    let mut doc = read_doc(&input)?;
    let mut frame = doc
        .frames
        .get(source)
        .context("source frame index is outside the asset")?
        .clone();
    frame.duration_ms = duration;
    doc.frames.push(frame);
    validate(&doc)?;
    write_doc(&output, &doc)?;
    println!(
        "Added frame {} from source frame {source}",
        doc.frames.len() - 1
    );
    Ok(())
}
fn set_frame_command(mut args: Vec<String>) -> Result<()> {
    let input = required_pos(&mut args, "input file")?;
    let output = option(&mut args, "--output", None)?.context("missing --output")?;
    let frame_index: usize = number(option(&mut args, "--frame", None)?, "--frame", None)?;
    let duration_ms: u32 = number(
        option(&mut args, "--duration-ms", None)?,
        "--duration-ms",
        None,
    )?;
    reject_extra(&args)?;
    let mut doc = read_doc(&input)?;
    let frame = doc
        .frames
        .get_mut(frame_index)
        .context("frame index is outside the asset")?;
    frame.duration_ms = duration_ms;
    validate(&doc)?;
    write_doc(&output, &doc)?;
    println!("Set frame {frame_index} duration to {duration_ms} ms");
    Ok(())
}
fn reject_extra(args: &[String]) -> Result<()> {
    if !args.is_empty() {
        bail!("unexpected argument(s): {}", args.join(" "));
    }
    Ok(())
}
