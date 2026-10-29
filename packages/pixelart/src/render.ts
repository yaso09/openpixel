import { validatePixelArt } from "./validate.js";
import type { PixelArtDocument, PixelFrame, RenderOptions } from "./types.js";

const escapeXml = (value: string): string => value.replace(/[&<>"']/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", "\"": "&quot;", "'": "&apos;" })[char]!);

export function renderFrameToSvg(document: PixelArtDocument, options: RenderOptions = {}): string {
  validatePixelArt(document);
  const frameIndex = options.frameIndex ?? 0;
  if (!Number.isInteger(frameIndex) || frameIndex < 0 || frameIndex >= document.frames.length) throw new RangeError(`frameIndex must be between 0 and ${document.frames.length - 1}`);
  const scale = options.pixelSize ?? 1;
  if (!Number.isFinite(scale) || scale <= 0) throw new RangeError("pixelSize must be a positive number");
  const frame = document.frames[frameIndex];
  const width = document.width * scale;
  const height = document.height * scale;
  const parts = [`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${document.width} ${document.height}" width="${width}" height="${height}" shape-rendering="crispEdges" role="img" aria-label="${escapeXml(options.title ?? document.name ?? "Pixel art")}">`];
  parts.push(`<title>${escapeXml(options.title ?? document.name ?? "Pixel art")}</title>`);
  if (options.background) {
    if (!/^#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?$/.test(options.background)) throw new TypeError("background must be a #RRGGBB or #RRGGBBAA color");
    parts.push(`<rect width="${document.width}" height="${document.height}" fill="${options.background}"/>`);
  }
  for (const layer of document.layers) {
    if (layer.visible === false) continue;
    const matrix = frame.layers.find((item) => item.layerId === layer.id)!.pixels;
    const rects: string[] = [];
    matrix.forEach((row, y) => row.forEach((color, x) => {
      if (color !== null) rects.push(`<rect x="${x}" y="${y}" width="1" height="1" fill="${color}"/>`);
    }));
    parts.push(`<g data-layer="${escapeXml(layer.id)}" opacity="${layer.opacity ?? 1}">${rects.join("")}</g>`);
  }
  parts.push("</svg>");
  return parts.join("");
}

export function renderToSvg(document: PixelArtDocument, options: RenderOptions = {}): string {
  return renderFrameToSvg(document, options);
}

export function getFrameDuration(frame: PixelFrame): number {
  return frame.durationMs;
}
