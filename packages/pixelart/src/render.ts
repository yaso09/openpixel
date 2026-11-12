import { render_json } from "../pkg/renderer_core.js";
import "./wasm.js";
import { PixelArtValidationError } from "./validate.js";
import type { PixelArtDocument, PixelFrame, RenderOptions } from "./types.js";

type WasmRenderError = { error: "validation"; issues: string[] } | { error: "range" | "type" | "options"; message: string };

function renderWithWasm(document: PixelArtDocument, options: RenderOptions): string {
  const frameIndex = options.frameIndex ?? 0;
  if (!Number.isInteger(frameIndex) || frameIndex < 0) {
    throw new RangeError(`frameIndex must be between 0 and ${(document?.frames?.length ?? 0) - 1}`);
  }
  const scale = options.pixelSize ?? 1;
  if (!Number.isInteger(scale) || scale <= 0) throw new RangeError("pixelSize must be a positive number");
  if (options.background && !/^#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?$/.test(options.background)) {
    throw new TypeError("background must be a #RRGGBB or #RRGGBBAA color");
  }

  let svg: string;
  try {
    svg = render_json(JSON.stringify(document), JSON.stringify(options));
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    let result: WasmRenderError;
    try {
      result = JSON.parse(message) as WasmRenderError;
    } catch {
      throw new PixelArtValidationError(["document must be an object"]);
    }
    if (result.error === "validation") throw new PixelArtValidationError(result.issues);
    if (result.error === "range") throw new RangeError(result.message);
    if (result.error === "type") throw new TypeError(result.message);
    throw new TypeError(result.message);
  }
  return svg;
}

export function renderFrameToSvg(document: PixelArtDocument, options: RenderOptions = {}): string {
  return renderWithWasm(document, options);
}

export function renderToSvg(document: PixelArtDocument, options: RenderOptions = {}): string {
  return renderFrameToSvg(document, options);
}

export function getFrameDuration(frame: PixelFrame): number {
  return frame.durationMs;
}
