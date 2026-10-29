use anyhow::{Context, Result};
use openpixel_cli::{read_doc, render_svg, write_doc, Document, Frame, FrameLayer, Layer};
use rmcp::{
    handler::server::router::tool::ToolRouter, handler::server::wrapper::Parameters, model::*,
    schemars::JsonSchema, tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler,
    ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env, net::SocketAddr};

#[derive(Clone)]
struct OpenPixelServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct PathArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct CreateArgs {
    /// File path to create
    path: String,
    /// Canvas width in pixels (1–2048)
    width: usize,
    /// Canvas height in pixels (1–1152)
    height: usize,
    /// Optional asset name
    name: Option<String>,
    /// Ordered layer identifiers; names are initially the same as IDs
    layer_ids: Option<Vec<String>>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct RenderArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Zero-based frame index
    frame_index: usize,
    /// SVG scale multiplier
    scale: Option<usize>,
    /// Optional #RRGGBB or #RRGGBBAA background
    background: Option<String>,
    /// Optional accessible SVG title
    title: Option<String>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct RenderAllArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// SVG scale multiplier
    scale: Option<usize>,
    /// Optional accessible SVG title
    title: Option<String>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct SetPixelArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Destination path
    output_path: String,
    /// Zero-based frame index
    frame_index: usize,
    /// Layer identifier
    layer_id: String,
    /// Zero-based horizontal coordinate
    x: usize,
    /// Zero-based vertical coordinate
    y: usize,
    /// #RRGGBB, #RRGGBBAA, or null to erase
    color: String,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct SetLayerArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Destination path
    output_path: String,
    /// Layer identifier
    layer_id: String,
    /// Optional new layer name
    name: Option<String>,
    /// Optional layer visibility
    visible: Option<bool>,
    /// Optional opacity from 0 to 1
    opacity: Option<f32>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct AddFrameArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Destination path
    output_path: String,
    /// Zero-based source frame index
    source_frame: Option<usize>,
    /// Duration for the copied frame in milliseconds
    duration_ms: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct SetFrameArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Destination path
    output_path: String,
    /// Zero-based frame index
    frame_index: usize,
    /// New frame duration, 1–60000 milliseconds
    duration_ms: u32,
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
struct SetLoopArgs {
    /// Existing OpenPixel JSON asset path
    path: String,
    /// Destination path
    output_path: String,
    /// Whether animation loops
    enabled: bool,
}
#[derive(Clone, Debug, Serialize)]
struct FrameSvg {
    frame_index: usize,
    duration_ms: u32,
    svg: String,
}

#[tool_router]
impl OpenPixelServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Create a transparent OpenPixel v1 asset with ordered layers and one frame."
    )]
    fn create_asset(
        &self,
        Parameters(args): Parameters<CreateArgs>,
    ) -> Result<CallToolResult, McpError> {
        if !(1..=2048).contains(&args.width) || !(1..=1152).contains(&args.height) || args.width as usize * args.height as usize > 2_359_296 {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                "width must be 1–2048, height 1–1152, and total canvas size at most 2,359,296 pixels",
            )]));
        }
        let layers = args
            .layer_ids
            .unwrap_or_else(|| vec!["base".into()])
            .into_iter()
            .map(|id| Layer {
                name: id.clone(),
                id,
                visible: true,
                opacity: 1.0,
            })
            .collect::<Vec<_>>();
        let entries = layers
            .iter()
            .map(|layer| FrameLayer {
                layer_id: layer.id.clone(),
                pixels: vec![vec![None; args.width]; args.height],
            })
            .collect();
        let doc = Document {
            version: 1,
            name: args.name,
            width: args.width,
            height: args.height,
            looping: true,
            layers,
            frames: vec![Frame {
                duration_ms: 100,
                layers: entries,
            }],
        };
        finish(write_doc(&args.path, &doc).map(|_| {
            format!(
                "Created {}x{} asset at {}",
                args.width, args.height, args.path
            )
        }))
    }
    #[tool(
        description = "Validate an OpenPixel v1 JSON file and report its dimensions, layers, and frames."
    )]
    fn validate_asset(
        &self,
        Parameters(args): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).map(|doc| {
            format!(
                "Valid OpenPixel v1 asset: {}x{}, {} layer(s), {} frame(s)",
                doc.width,
                doc.height,
                doc.layers.len(),
                doc.frames.len()
            )
        }))
    }
    #[tool(description = "Inspect asset metadata, frame timings, loop setting, and color palette.")]
    fn inspect_asset(
        &self,
        Parameters(args): Parameters<PathArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).map(|doc| {
            let mut palette = BTreeMap::<String, usize>::new();
            for frame in &doc.frames { for layer in &frame.layers { for row in &layer.pixels { for pixel in row { if let Some(color) = pixel { *palette.entry(color.clone()).or_default() += 1; } } } } }
            serde_json::to_string_pretty(&serde_json::json!({"name": doc.name, "width": doc.width, "height": doc.height, "loop": doc.looping, "layers": doc.layers, "frames": doc.frames.iter().enumerate().map(|(index, frame)| serde_json::json!({"index": index, "durationMs": frame.duration_ms})).collect::<Vec<_>>(), "palette": palette})).unwrap_or_else(|_| "{}".into())
        }))
    }
    #[tool(
        description = "Render a selected frame as crisp SVG text using the OpenPixel renderer options."
    )]
    fn render_frame(
        &self,
        Parameters(args): Parameters<RenderArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|doc| {
            render_svg(
                &doc,
                args.frame_index,
                args.scale.unwrap_or(1),
                args.background.as_deref(),
                args.title.as_deref(),
            )
        }))
    }
    #[tool(
        description = "Render every animation frame as SVG; each result includes its index and durationMs."
    )]
    fn render_all_frames(
        &self,
        Parameters(args): Parameters<RenderAllArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|doc| {
            let scale = args.scale.unwrap_or(1);
            let frames = doc
                .frames
                .iter()
                .enumerate()
                .map(|(index, frame)| {
                    Ok(FrameSvg {
                        frame_index: index,
                        duration_ms: frame.duration_ms,
                        svg: render_svg(&doc, index, scale, None, args.title.as_deref())?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            serde_json::to_string(&frames).context("could not serialize SVG frames")
        }))
    }
    #[tool(
        description = "Set a pixel to a hex color or null for transparency and write the result to output_path."
    )]
    fn set_pixel(
        &self,
        Parameters(args): Parameters<SetPixelArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|mut doc| {
            if args.x >= doc.width || args.y >= doc.height {
                anyhow::bail!("coordinates are outside the canvas");
            }
            let frame = doc
                .frames
                .get_mut(args.frame_index)
                .context("frame index is outside the asset")?;
            let layer = frame
                .layers
                .iter_mut()
                .find(|item| item.layer_id == args.layer_id)
                .context("layer id not found")?;
            layer.pixels[args.y][args.x] = if args.color == "null" {
                None
            } else {
                if !openpixel_cli::valid_color(&args.color) {
                    anyhow::bail!("color must be #RRGGBB, #RRGGBBAA, or null");
                }
                Some(args.color)
            };
            write_doc(&args.output_path, &doc).map(|_| {
                format!(
                    "Updated pixel ({},{}) in frame {}",
                    args.x, args.y, args.frame_index
                )
            })
        }))
    }
    #[tool(description = "Change layer name, visibility, and/or opacity in an OpenPixel asset.")]
    fn set_layer(
        &self,
        Parameters(args): Parameters<SetLayerArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|mut doc| {
            if args.name.is_none() && args.visible.is_none() && args.opacity.is_none() {
                anyhow::bail!("provide a name, visible, or opacity update");
            }
            let layer = doc
                .layers
                .iter_mut()
                .find(|layer| layer.id == args.layer_id)
                .context("layer id not found")?;
            if let Some(name) = args.name {
                layer.name = name;
            }
            if let Some(visible) = args.visible {
                layer.visible = visible;
            }
            if let Some(opacity) = args.opacity {
                layer.opacity = opacity;
            }
            write_doc(&args.output_path, &doc).map(|_| format!("Updated layer {}", args.layer_id))
        }))
    }
    #[tool(description = "Duplicate a frame to the end of the animation with a chosen duration.")]
    fn add_frame(
        &self,
        Parameters(args): Parameters<AddFrameArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|mut doc| {
            let source = args.source_frame.unwrap_or(0);
            let mut frame = doc
                .frames
                .get(source)
                .context("source frame is outside the asset")?
                .clone();
            frame.duration_ms = args.duration_ms.unwrap_or(100);
            doc.frames.push(frame);
            write_doc(&args.output_path, &doc)
                .map(|_| format!("Added frame {}", doc.frames.len() - 1))
        }))
    }
    #[tool(description = "Change the duration of an existing frame in milliseconds.")]
    fn set_frame_duration(
        &self,
        Parameters(args): Parameters<SetFrameArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|mut doc| {
            doc.frames
                .get_mut(args.frame_index)
                .context("frame index is outside the asset")?
                .duration_ms = args.duration_ms;
            write_doc(&args.output_path, &doc).map(|_| {
                format!(
                    "Set frame {} duration to {} ms",
                    args.frame_index, args.duration_ms
                )
            })
        }))
    }
    #[tool(description = "Enable or disable looping for an OpenPixel animation.")]
    fn set_loop(
        &self,
        Parameters(args): Parameters<SetLoopArgs>,
    ) -> Result<CallToolResult, McpError> {
        finish(read_doc(&args.path).and_then(|mut doc| {
            doc.looping = args.enabled;
            write_doc(&args.output_path, &doc).map(|_| format!("Looping set to {}", args.enabled))
        }))
    }
}

fn finish(result: anyhow::Result<String>) -> Result<CallToolResult, McpError> {
    match result {
        Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
        Err(error) => Ok(CallToolResult::error(vec![ContentBlock::text(
            error.to_string(),
        )])),
    }
}
#[tool_handler]
impl ServerHandler for OpenPixelServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("openpixel-mcp", env!("CARGO_PKG_VERSION")))
            .with_instructions("OpenPixel tools create, inspect, validate, edit, and render pixel art assets in the shared v1 JSON format. File edits use explicit output paths.")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let transport = args.next().unwrap_or_else(|| "stdio".into());
    match transport.as_str() {
        "stdio" => {
            if args.next().is_some() {
                anyhow::bail!("usage: openpixel-mcp [stdio|http]");
            }
            let service = OpenPixelServer::new()
                .serve(rmcp::transport::stdio())
                .await?;
            service.waiting().await?;
        }
        "http" => {
            let address = args
                .next()
                .or_else(|| env::var("OPENPIXEL_MCP_ADDR").ok())
                .unwrap_or_else(|| "127.0.0.1:8000".into());
            if args.next().is_some() {
                anyhow::bail!("usage: openpixel-mcp http [host:port]");
            }
            let address: SocketAddr = address.parse().context("invalid bind address")?;
            let service = rmcp::transport::streamable_http_server::tower::StreamableHttpService::new(
                || Ok(OpenPixelServer::new()),
                rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default().into(),
                rmcp::transport::StreamableHttpServerConfig::default(),
            );
            let router = axum::Router::new().nest_service("/mcp", service);
            let listener = tokio::net::TcpListener::bind(address).await?;
            eprintln!("OpenPixel MCP Streamable HTTP/SSE listening at http://{address}/mcp");
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    tokio::signal::ctrl_c().await.ok();
                })
                .await?;
        }
        _ => anyhow::bail!("usage: openpixel-mcp [stdio|http [host:port]]"),
    }
    Ok(())
}
