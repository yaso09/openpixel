import type { PixelArtDocument } from "./types.js";

export class PixelArtValidationError extends Error {
  readonly issues: string[];
  constructor(issues: string[]) {
    super(`Invalid OpenPixel document: ${issues.join("; ")}`);
    this.name = "PixelArtValidationError";
    this.issues = issues;
  }
}

export function validatePixelArt(value: unknown): asserts value is PixelArtDocument {
  const issues: string[] = [];
  const doc = value as Partial<PixelArtDocument> | null;
  if (!doc || typeof doc !== "object") throw new PixelArtValidationError(["document must be an object"]);
  if (doc.version !== 1) issues.push("version must be 1");
  if (doc.revision !== undefined && (!Number.isInteger(doc.revision) || doc.revision < 1)) issues.push("revision must be an integer of at least 1");
  if (!Number.isInteger(doc.width) || !doc.width || doc.width < 1 || doc.width > 4096) issues.push("width must be an integer from 1 to 4096");
  if (!Number.isInteger(doc.height) || !doc.height || doc.height < 1 || doc.height > 2304) issues.push("height must be an integer from 1 to 2304");
  if (Number.isInteger(doc.width) && Number.isInteger(doc.height) && doc.width! * doc.height! > 9_437_184) issues.push("canvas must not exceed 9,437,184 pixels");
  if (!Array.isArray(doc.layers) || doc.layers.length === 0) issues.push("layers must be a non-empty array");
  if (!Array.isArray(doc.frames) || doc.frames.length === 0) issues.push("frames must be a non-empty array");

  const layerIds = new Set<string>();
  for (const [index, layer] of (Array.isArray(doc.layers) ? doc.layers : []).entries()) {
    if (!layer || typeof layer.id !== "string" || !/^[a-zA-Z0-9_-]+$/.test(layer.id)) issues.push(`layers[${index}].id is invalid`);
    else if (layerIds.has(layer.id)) issues.push(`duplicate layer id: ${layer.id}`);
    else layerIds.add(layer.id);
    if (!layer || typeof layer.name !== "string" || !layer.name.trim()) issues.push(`layers[${index}].name is required`);
    if (layer?.opacity !== undefined && (typeof layer.opacity !== "number" || layer.opacity < 0 || layer.opacity > 1)) issues.push(`layers[${index}].opacity must be from 0 to 1`);
  }

  for (const [frameIndex, frame] of (Array.isArray(doc.frames) ? doc.frames : []).entries()) {
    if (!frame || !Number.isInteger(frame.durationMs) || frame.durationMs < 1 || frame.durationMs > 60000) issues.push(`frames[${frameIndex}].durationMs must be from 1 to 60000`);
    if (!frame || !Array.isArray(frame.layers)) { issues.push(`frames[${frameIndex}].layers must be an array`); continue; }
    const seen = new Set<string>();
    for (const entry of frame.layers) {
      if (!entry || !layerIds.has(entry.layerId)) { issues.push(`frames[${frameIndex}] references an unknown layer`); continue; }
      if (seen.has(entry.layerId)) issues.push(`frames[${frameIndex}] repeats layer ${entry.layerId}`);
      seen.add(entry.layerId);
      if (!Array.isArray(entry.pixels) || entry.pixels.length !== doc.height) { issues.push(`frames[${frameIndex}].${entry.layerId} must have ${doc.height} rows`); continue; }
      entry.pixels.forEach((row, y) => {
        if (!Array.isArray(row) || row.length !== doc.width) { issues.push(`frames[${frameIndex}].${entry.layerId} row ${y} must have ${doc.width} columns`); return; }
        row.forEach((pixel, x) => {
          if (pixel !== null && (typeof pixel !== "string" || !/^#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?$/.test(pixel))) issues.push(`frames[${frameIndex}].${entry.layerId}[${y}][${x}] must be null or #RRGGBB[AA]`);
        });
      });
    }
    for (const id of layerIds) if (!seen.has(id)) issues.push(`frames[${frameIndex}] is missing layer ${id}`);
  }
  if (issues.length) throw new PixelArtValidationError(issues);
}
