use serde::Deserialize;
use serde_json::{json, Value};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct Document {
    version: u32,
    #[serde(default)]
    revision: Option<u64>,
    width: u32,
    height: u32,
    #[serde(default)]
    name: Option<String>,
    layers: Vec<Layer>,
    frames: Vec<Frame>,
}

#[derive(Deserialize)]
struct Layer {
    id: String,
    name: String,
    #[serde(default)]
    visible: Option<bool>,
    #[serde(default)]
    opacity: Option<f64>,
}

#[derive(Deserialize)]
struct Frame {
    #[serde(rename = "durationMs")]
    duration_ms: u32,
    layers: Vec<FrameLayer>,
}

#[derive(Deserialize)]
struct FrameLayer {
    #[serde(rename = "layerId")]
    layer_id: String,
    pixels: Vec<Vec<Option<String>>>,
}

#[derive(Deserialize, Default)]
struct Options {
    #[serde(rename = "frameIndex")]
    frame_index: Option<usize>,
    #[serde(rename = "pixelSize")]
    pixel_size: Option<f64>,
    background: Option<String>,
    title: Option<String>,
}

fn escaped(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn valid_color(value: &str) -> bool {
    let bytes = value.as_bytes();
    matches!(bytes.len(), 7 | 9) && bytes[0] == b'#' && bytes[1..].iter().all(u8::is_ascii_hexdigit)
}

fn number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn validation_issues(value: &Value) -> Vec<String> {
    let mut issues = Vec::new();
    let Some(doc) = value.as_object() else {
        return vec!["document must be an object".into()];
    };

    if doc.get("version").and_then(Value::as_u64) != Some(1) {
        issues.push("version must be 1".into());
    }
    if let Some(revision) = doc.get("revision") {
        if !revision.as_u64().is_some_and(|n| n >= 1) {
            issues.push("revision must be an integer of at least 1".into());
        }
    }
    let width = doc.get("width").and_then(Value::as_u64);
    let height = doc.get("height").and_then(Value::as_u64);
    if !width.is_some_and(|n| (1..=4096).contains(&n)) {
        issues.push("width must be an integer from 1 to 4096".into());
    }
    if !height.is_some_and(|n| (1..=2304).contains(&n)) {
        issues.push("height must be an integer from 1 to 2304".into());
    }
    if let (Some(w), Some(h)) = (width, height) {
        if w.saturating_mul(h) > 9_437_184 {
            issues.push("canvas must not exceed 9,437,184 pixels".into());
        }
    }

    let layers = doc.get("layers").and_then(Value::as_array);
    if !layers.is_some_and(|items| !items.is_empty()) {
        issues.push("layers must be a non-empty array".into());
    }
    let frames = doc.get("frames").and_then(Value::as_array);
    if !frames.is_some_and(|items| !items.is_empty()) {
        issues.push("frames must be a non-empty array".into());
    }

    let mut ids = Vec::<String>::new();
    if let Some(layers) = layers {
        for (index, layer) in layers.iter().enumerate() {
            let id = layer.get("id").and_then(Value::as_str);
            match id {
                Some(id)
                    if !id.is_empty()
                        && id
                            .bytes()
                            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-') =>
                {
                    if ids.iter().any(|seen| seen == id) {
                        issues.push(format!("duplicate layer id: {id}"));
                    } else {
                        ids.push(id.to_string());
                    }
                }
                _ => issues.push(format!("layers[{index}].id is invalid")),
            }
            if !layer
                .get("name")
                .and_then(Value::as_str)
                .is_some_and(|name| !name.trim().is_empty())
            {
                issues.push(format!("layers[{index}].name is required"));
            }
            if let Some(opacity) = layer.get("opacity") {
                let valid = opacity.as_f64().is_some_and(|n| (0.0..=1.0).contains(&n));
                if !valid {
                    issues.push(format!("layers[{index}].opacity must be from 0 to 1"));
                }
            }
        }
    }

    if let Some(frames) = frames {
        for (frame_index, frame) in frames.iter().enumerate() {
            let duration = frame.get("durationMs").and_then(Value::as_u64);
            if !duration.is_some_and(|n| (1..=60_000).contains(&n)) {
                issues.push(format!(
                    "frames[{frame_index}].durationMs must be from 1 to 60000"
                ));
            }
            let Some(entries) = frame.get("layers").and_then(Value::as_array) else {
                issues.push(format!("frames[{frame_index}].layers must be an array"));
                continue;
            };
            let mut seen = Vec::<String>::new();
            for entry in entries {
                let id = entry.get("layerId").and_then(Value::as_str);
                let Some(id) = id.filter(|id| ids.iter().any(|known| known == *id)) else {
                    issues.push(format!("frames[{frame_index}] references an unknown layer"));
                    continue;
                };
                if seen.iter().any(|known| known == id) {
                    issues.push(format!("frames[{frame_index}] repeats layer {id}"));
                }
                seen.push(id.to_string());
                let Some(matrix) = entry.get("pixels").and_then(Value::as_array) else {
                    issues.push(format!(
                        "frames[{frame_index}].{id} must have {} rows",
                        height.unwrap_or(0)
                    ));
                    continue;
                };
                if let Some(h) = height {
                    if matrix.len() as u64 != h {
                        issues.push(format!("frames[{frame_index}].{id} must have {h} rows"));
                        continue;
                    }
                    for (y, row) in matrix.iter().enumerate() {
                        let Some(row) = row.as_array() else {
                            issues.push(format!(
                                "frames[{frame_index}].{id} row {y} must have {} columns",
                                width.unwrap_or(0)
                            ));
                            continue;
                        };
                        if Some(row.len() as u64) != width {
                            issues.push(format!(
                                "frames[{frame_index}].{id} row {y} must have {} columns",
                                width.unwrap_or(0)
                            ));
                            continue;
                        }
                        for (x, pixel) in row.iter().enumerate() {
                            if !pixel.is_null() && !pixel.as_str().is_some_and(valid_color) {
                                issues.push(format!("frames[{frame_index}].{id}[{y}][{x}] must be null or #RRGGBB[AA]"));
                            }
                        }
                    }
                }
            }
            for id in &ids {
                if !seen.iter().any(|known| known == id) {
                    issues.push(format!("frames[{frame_index}] is missing layer {id}"));
                }
            }
        }
    }
    issues
}

fn render(document_json: &str, options_json: &str) -> Value {
    let value: Value = match serde_json::from_str(document_json) {
        Ok(value) => value,
        Err(_) => return json!({"error": "validation", "issues": ["document must be an object"]}),
    };
    let issues = validation_issues(&value);
    if !issues.is_empty() {
        return json!({"error": "validation", "issues": issues});
    }
    let document: Document = match serde_json::from_value(value) {
        Ok(document) => document,
        Err(_) => {
            return json!({"error": "validation", "issues": ["document contains invalid fields"]})
        }
    };
    let options: Options = match serde_json::from_str(options_json) {
        Ok(options) => options,
        Err(_) => return json!({"error": "options", "message": "render options are invalid"}),
    };
    let frame_index = options.frame_index.unwrap_or(0);
    if frame_index >= document.frames.len() {
        return json!({"error": "range", "message": format!("frameIndex must be between 0 and {}", document.frames.len() - 1)});
    }
    let scale = options.pixel_size.unwrap_or(1.0);
    if scale <= 0.0 || !scale.is_finite() || scale.fract() != 0.0 {
        return json!({"error": "range", "message": "pixelSize must be a positive number"});
    }
    if let Some(background) = &options.background {
        if !background.is_empty() && !valid_color(background) {
            return json!({"error": "type", "message": "background must be a #RRGGBB or #RRGGBBAA color"});
        }
    }

    let frame = &document.frames[frame_index];
    let title = escaped(
        options
            .title
            .as_deref()
            .or(document.name.as_deref())
            .unwrap_or("Pixel art"),
    );
    let mut parts = vec![format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" shape-rendering=\"crispEdges\" role=\"img\" aria-label=\"{}\">",
        document.width, document.height, number(document.width as f64 * scale), number(document.height as f64 * scale), title
    )];
    parts.push(format!("<title>{title}</title>"));
    if let Some(background) = options.background.filter(|value| !value.is_empty()) {
        parts.push(format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>",
            document.width, document.height, background
        ));
    }

    for layer in &document.layers {
        if layer.visible == Some(false) {
            continue;
        }
        let entry = frame
            .layers
            .iter()
            .find(|entry| entry.layer_id == layer.id)
            .expect("validated layer matrix");
        let mut paths_by_color: Vec<(String, Vec<String>)> = Vec::new();
        for (y, row) in entry.pixels.iter().enumerate() {
            let mut start = 0;
            while start < row.len() {
                let Some(color) = row[start].as_deref() else {
                    start += 1;
                    continue;
                };
                let mut end = start + 1;
                while end < row.len() && row[end].as_deref() == Some(color) {
                    end += 1;
                }
                let item = if let Some(index) =
                    paths_by_color.iter().position(|(known, _)| known == color)
                {
                    &mut paths_by_color[index].1
                } else {
                    paths_by_color.push((color.to_string(), Vec::new()));
                    &mut paths_by_color.last_mut().unwrap().1
                };
                let run_width = end - start;
                item.push(format!("M{start} {y}h{run_width}v1h-{run_width}z"));
                start = end;
            }
        }
        let paths = paths_by_color
            .into_iter()
            .map(|(color, commands)| {
                format!("<path fill=\"{color}\" d=\"{}\"/>", commands.join(""))
            })
            .collect::<String>();
        let opacity = number(layer.opacity.unwrap_or(1.0));
        parts.push(format!(
            "<g data-layer=\"{}\" opacity=\"{opacity}\">{paths}</g>",
            escaped(&layer.id)
        ));
    }
    parts.push("</svg>".into());
    let _ = (
        document.version,
        document.revision,
        &document.layers[0].name,
        frame.duration_ms,
    );
    json!({"svg": parts.join("")})
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn render_json(document_json: &str, options_json: &str) -> String {
    render(document_json, options_json).to_string()
}

pub fn render_json_native(document_json: &str, options_json: &str) -> String {
    render(document_json, options_json).to_string()
}
