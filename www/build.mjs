import { writeFile, mkdir } from 'node:fs/promises';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { renderFrameToSvg, validatePixelArt } from '@openpixel/renderer';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const out = resolve(root, 'www/assets/landscape');
const scale = 2;
const sourceWidth = 96;
const sourceHeight = 54;
const width = sourceWidth * scale;
const height = sourceHeight * scale;
const C = {
  sky: '#253c4a', dusk: '#49665c', far: '#71836a', snow: '#c4c69b',
  hill: '#354f49', pine: '#203d3c', deep: '#182f35', water: '#284b50',
  shine: '#6b8981', glow: '#f2c879', reed: '#82915c', ink: '#162a31'
};
const blank = () => Array.from({ length: height }, () => Array(width).fill(null));
const put = (matrix, x, y, color) => {
  for (let dy = 0; dy < scale; dy++) for (let dx = 0; dx < scale; dx++) {
    const xx = x * scale + dx, yy = y * scale + dy;
    if (xx >= 0 && xx < width && yy >= 0 && yy < height) matrix[yy][xx] = color;
  }
};
const putFine = (matrix, x, y, color) => {
  if (x >= 0 && x < width && y >= 0 && y < height) matrix[y][x] = color;
};
const row = (matrix, y, start, end, color) => {
  for (let x = start; x <= end; x++) put(matrix, x, y, color);
};
const fineRow = (matrix, y, start, end, color) => {
  for (let x = start; x <= end; x++) putFine(matrix, x, y, color);
};
const fill = (matrix, x, y, w, h, color) => {
  for (let yy = y; yy < y + h; yy++) row(matrix, yy, x, x + w - 1, color);
};
const ids = ['sky', 'sun', 'distant', 'mountains', 'forest', 'lake', 'foreground', 'sparkles'];
const layers = Object.fromEntries(ids.map((id) => [id, blank()]));

for (let y = 0; y < sourceHeight; y++) row(layers.sky, y, 0, sourceWidth - 1, y < 16 ? C.sky : y < 23 ? C.dusk : C.deep);
for (const [x, y, w] of [[12, 9, 9], [56, 6, 12], [75, 13, 7]]) {
  row(layers.sky, y, x + 2, x + w - 3, C.far);
  row(layers.sky, y + 1, x, x + w - 1, C.far);
  row(layers.sky, y + 2, x + 1, x + w - 2, C.far);
}
fill(layers.sun, 73, 14, 6, 6, C.glow);
fill(layers.sun, 71, 16, 10, 2, C.glow);
for (let y = 14; y < 29; y++) {
  const edge = 43 - Math.abs(y - 22) * 2;
  row(layers.distant, y, 0, edge, C.far);
  row(layers.distant, y, 95 - edge, 95, C.far);
}
for (const [x, peak] of [[24, 14], [72, 15]]) {
  for (let dy = 0; dy < 8; dy++) row(layers.distant, peak + dy, x - dy, x + dy, dy < 3 ? C.snow : C.far);
}
for (let y = 20; y < 34; y++) {
  const edge = 28 - Math.abs(y - 26) * 2;
  row(layers.mountains, y, 0, edge, C.hill);
  row(layers.mountains, y, 95 - edge, 95, C.hill);
}
const tree = (x, base, size, color) => {
  for (let level = 0; level < size; level++) {
    const half = Math.floor(level / 2);
    row(layers.forest, base - size + level, x - half, x + half, color);
  }
  fill(layers.forest, x - 1, base - 1, 3, 3, C.ink);
};
for (let x = 1; x < 37; x += 5) tree(x, 34, 8 + x % 3, x % 2 ? C.pine : C.hill);
for (let x = 59; x < 96; x += 5) tree(x, 35, 9 + x % 3, x % 2 ? C.pine : C.hill);
fill(layers.lake, 0, 31, sourceWidth, 23, C.water);
for (let y = 34; y < sourceHeight; y++) {
  row(layers.foreground, y, 0, 10 + y % 4, C.ink);
  row(layers.foreground, y, 85 - y % 4, 95, C.ink);
}
for (let x = 0; x < 35; x += 4) {
  row(layers.foreground, 31 + x % 3, x, x + 3, C.reed);
  put(layers.foreground, x + 1, 30, C.reed);
}
for (let x = 62; x < 96; x += 4) {
  row(layers.foreground, 32 + x % 2, x, x + 3, C.reed);
  put(layers.foreground, x + 2, 31, C.reed);
}
for (let x = 8; x < 26; x += 6) {
  put(layers.foreground, x, 32, C.reed);
  put(layers.foreground, x + 1, 30, C.reed);
}
for (let x = 72; x < 89; x += 5) {
  put(layers.foreground, x, 33, C.reed);
  put(layers.foreground, x - 1, 31, C.reed);
}

// Fine single-pixel accents use the new resolution for stars, branches, and reflections.
for (let i = 0; i < 54; i++) {
  const x = (i * 37 + 11) % (width - 12) + 6;
  const y = (i * 19 + 7) % 38 + 5;
  if (y < 39) putFine(layers.sparkles, x, y, i % 4 === 0 ? C.glow : C.snow);
}
for (let i = 0; i < 86; i++) {
  const x = (i * 29 + 17) % (width - 24) + 12;
  const y = 48 + (i * 17) % 18;
  putFine(layers.forest, x, y, i % 3 === 0 ? C.reed : C.pine);
}
for (let i = 0; i < 100; i++) {
  const x = (i * 31 + 21) % (width - 16) + 8;
  const y = 64 + (i * 13) % 40;
  putFine(layers.lake, x, y, i % 3 === 0 ? C.shine : C.dusk);
}

const durations = [900, 700, 1000, 800];
const frame = (phase) => ids.map((layerId) => {
  const pixels = layers[layerId].map((line) => [...line]);
  if (layerId === 'lake') {
    for (let i = 0; i < 9; i++) {
      const x = (i * 13 + phase * 7) % 86 + 4;
      const y = 35 + (i * 5 + phase * 3) % 16;
      fineRow(pixels, y * scale, x * scale, (x + i % 3 + 1) * scale - 1, (i + phase) % 2 ? C.shine : C.dusk);
    }
  }
  if (layerId === 'sparkles') {
    for (const [x, y] of [[11, 28], [32, 36], [52, 30], [66, 40], [86, 26], [44, 46]]) {
      put(pixels, x + (phase + x) % 3 - 1, y, C.glow);
    }
  }
  if (layerId === 'sky' && phase) {
    const base = layers.sky;
    for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
      if (base[y][x] === C.far) pixels[y][x] = C.sky;
    }
    for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
      if (base[y][x] === C.far) pixels[y][(x + phase * 2) % width] = C.far;
    }
  }
  return { layerId, pixels };
});
const asset = {
  version: 1, name: 'Dusk over the quiet lake', width, height, loop: true,
  layers: ids.map((id, i) => ({ id, name: ['Evening sky', 'Last light', 'Blue mountains', 'Ridge', 'Pine forest', 'Moving water', 'Near shore', 'Fireflies'][i], visible: true, opacity: 1 })),
  frames: durations.map((durationMs, phase) => ({ durationMs, layers: frame(phase) }))
};
validatePixelArt(asset);
await mkdir(out, { recursive: true });
await writeFile(resolve(out, 'landscape.json'), `${JSON.stringify(asset)}\n`, 'utf8');
for (let i = 0; i < asset.frames.length; i++) {
  const svg = renderFrameToSvg(asset, { frameIndex: i, pixelSize: 6, title: 'OpenPixel pixel art mountain lake' });
  await writeFile(resolve(out, `frame-${i + 1}.svg`), svg, 'utf8');
}
console.log(`Rendered ${width}×${height} pixel-art landscape (${asset.frames.length} frames)`);
