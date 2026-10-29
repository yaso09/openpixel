import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { renderFrameToSvg, validatePixelArt } from '@openpixel/renderer';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const out = resolve(root, 'www/assets/landscape');
const width = 2048, height = 1152;
const colors = {
  sky0:'#111d35', sky1:'#1c2d49', sky2:'#35435d', sky3:'#66526a', sky4:'#b36b70', sky5:'#e39a78',
  cloud:'#d58d83', cloudHi:'#f1b58c', cloudShade:'#795d71', star:'#f6d8ad', moon:'#ffd69a', moonShade:'#e8a681',
  far0:'#465b6d', far1:'#536274', far2:'#667078', snow0:'#d2c6bd', snow1:'#aeb4b5', snowShade:'#828e9d',
  peak0:'#35495c', peak1:'#405261', peak2:'#52616a', peakDark:'#283d4c', snowWhite:'#e2d8ce',
  hill0:'#354e50', hill1:'#294449', pine0:'#213c40', pine1:'#1a343a', pine2:'#142d35', trunk:'#302f39',
  water0:'#34545b', water1:'#2c4a53', water2:'#26424d', water3:'#203b47', waterHi:'#698078', waterDim:'#46666a',
  reflect0:'#d59b7e', reflect1:'#bb8176', reflect2:'#8e6b70', shore:'#1b3439', reed:'#72806a', rock:'#263b40'
};
const pixels = Array.from({ length: height }, () => Array(width).fill(null));
const paint = (x, y, color) => { if (x >= 0 && x < width && y >= 0 && y < height) pixels[y][x] = color; };
const span = (y, left, right, color) => {
  const a = Math.max(0, Math.ceil(left)), b = Math.min(width - 1, Math.floor(right));
  if (y < 0 || y >= height || a > b) return;
  pixels[y].fill(color, a, b + 1);
};
const rect = (x, y, w, h, color) => { for (let yy = y; yy < y + h; yy++) span(yy, x, x + w - 1, color); };
let seed = 0x0face;
const random = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
const noise = (x, y, mod = 1) => ((Math.floor(x / 6) * 374761393 + Math.floor(y / 4) * 668265263 + 1442695041) >>> 0) % mod;

// A banded dusk sky, drawn pixel by pixel with quiet chromatic dither.
const skyStops = [[0,colors.sky0],[210,colors.sky1],[370,colors.sky2],[490,colors.sky3],[590,colors.sky4],[700,colors.sky5],[780,colors.sky4]];
const skyAt = (y) => {
  for (let i = 1; i < skyStops.length; i++) if (y < skyStops[i][0]) return skyStops[i - 1][1];
  return colors.sky4;
};
for (let y = 0; y < 790; y++) {
  const base = skyAt(y);
  span(y, 0, width - 1, base);
  const count = y < 420 ? 13 : 7;
  for (let i = 0; i < count; i++) {
    const x = Math.floor(random() * width), length = 1 + Math.floor(random() * (y < 350 ? 3 : 8));
    span(y, x, x + length, y < 530 ? (i % 4 ? colors.sky1 : colors.sky2) : (i % 2 ? colors.sky4 : colors.sky5));
  }
}
// Hundreds of small, restrained stars and a few cross-shaped glints.
for (let i = 0; i < 180; i++) {
  const x = Math.floor(random() * width), y = Math.floor(random() * 460);
  if (y > 360 && x > 1120 && x < 1650) continue;
  paint(x, y, i % 7 === 0 ? colors.cloudHi : colors.star);
  if (i % 19 === 0) { paint(x - 1, y, colors.sky2); paint(x + 1, y, colors.sky2); paint(x, y - 1, colors.sky2); paint(x, y + 1, colors.sky2); }
}
// Thin stepped clouds catch the warm light over the ridges.
for (const [x,y,w,h] of [[160,270,410,32],[485,225,300,22],[830,310,370,30],[1450,235,380,26],[1190,390,310,20],[80,420,230,17]]) {
  for (let yy = 0; yy < h; yy++) {
    const inset = Math.floor(Math.abs(yy - h / 2) * .72);
    span(y + yy, x + inset, x + w - inset, yy < h * .35 ? colors.cloudShade : yy > h * .7 ? colors.cloud : colors.cloudHi);
  }
  for (let xx = x + 30; xx < x + w - 30; xx += 72) {
    rect(xx, y - 5, 35, 5, colors.cloudHi);
    rect(xx + 14, y + h, 44, 4, colors.cloudShade);
  }
}
// Large setting sun with horizontal pixel cuts and a broad reflected glow.
const sunX = 1392, sunY = 394, sunR = 104;
for (let y = -sunR; y <= sunR; y++) {
  const half = Math.floor(Math.sqrt(sunR * sunR - y * y));
  span(sunY + y, sunX - half, sunX + half, Math.abs(y) > 70 ? colors.moonShade : colors.moon);
}
for (let y = sunY + 17; y < sunY + 75; y += 14) span(y, sunX - 72 + (y % 3) * 5, sunX + 77 - (y % 4) * 7, colors.sky4);

// Distant mountain chain: stepped ridges, cool shadow planes, and broken snow caps.
const farPeaks = [[-80,515],[150,422],[325,468],[560,315],[760,444],[1000,356],[1195,465],[1450,335],[1660,430],[1870,360],[2130,490]];
const mainPeaks = [[-60,650],[170,555],[360,612],[630,410],[830,560],[1080,448],[1280,570],[1510,392],[1710,540],[1910,425],[2110,600]];
const topAt = (points, x) => {
  let i = 1; while (i < points.length && points[i][0] < x) i++;
  const [x0,y0] = points[i - 1], [x1,y1] = points[Math.min(i, points.length - 1)];
  return Math.round(y0 + (y1 - y0) * (x - x0) / (x1 - x0));
};
for (let x = 0; x < width; x++) {
  const farTop = topAt(farPeaks, x);
  const mainTop = topAt(mainPeaks, x);
  for (let y = farTop; y < 728; y++) {
    const depth = y - farTop, ridge = Math.max(0, topAt(farPeaks, Math.max(0,x-2)) - topAt(farPeaks, Math.min(width-1,x+2)));
    let c = ridge > 0 ? colors.far1 : colors.far0;
    if (depth < 110 && farTop < 430) c = depth < 22 ? colors.snow0 : depth < 42 ? colors.snow1 : depth < 60 && noise(x,y,5) === 0 ? colors.snowShade : c;
    else if (noise(x,y,19) === 0) c = colors.far2;
    paint(x,y,c);
  }
  for (let y = mainTop; y < 776; y++) {
    const depth = y - mainTop;
    const slope = topAt(mainPeaks, Math.min(width - 1, x + 12)) - topAt(mainPeaks, Math.max(0, x - 12));
    let c = slope < 0 ? colors.peak1 : colors.peak0;
    if (depth < 18 && mainTop < 500) c = colors.snowWhite;
    else if (depth < 58 && mainTop < 510 && noise(x,y,7) < 3) c = colors.snow1;
    else if ((x < 1040 && noise(x,y,11) < 2) || (x > 1340 && noise(x,y,9) < 2)) c = colors.peakDark;
    else if (noise(x,y,23) === 0) c = colors.peak2;
    paint(x,y,c);
  }
}
// Glacier seams and angled rock facets follow the mountain faces in stepped runs.
for (let i = 0; i < 42; i++) {
  const x0 = 80 + Math.floor(random() * 1850), y0 = 455 + Math.floor(random() * 150), slope = random() < .5 ? -1 : 1;
  const length = 24 + Math.floor(random() * 72);
  for (let n = 0; n < length; n += 3) {
    const y = y0 + Math.floor(n * .48), x = x0 + slope * n;
    if (y < 760 && pixels[y][x] && (pixels[y][x] === colors.peak0 || pixels[y][x] === colors.peak1)) span(y, x, x + 2, n % 2 ? colors.peak2 : colors.peakDark);
  }
}

// Forested foothills overlap the mountain foot and frame the water.
const hillLine = (x) => 667 + Math.round(30 * Math.sin(x / 151) + 17 * Math.sin(x / 61) + (noise(x,17,19) - 9));
for (let x = 0; x < width; x++) {
  const top = hillLine(x);
  for (let y = top; y < 873; y++) paint(x,y, y < top + 55 ? colors.hill0 : (noise(x,y,6) < 3 ? colors.hill1 : colors.pine0));
}
const pine = (cx, base, height, tint) => {
  rect(cx - 3, base - 10, 7, 12, colors.trunk);
  const tiers = Math.floor(height / 13);
  for (let t = 0; t < tiers; t++) {
    const y = base - height + t * 13;
    const half = 4 + t * 3 + Math.floor(t / 2);
    for (let yy = 0; yy < 13; yy++) {
      const widthAt = Math.max(2, half - Math.floor(yy / 3));
      span(y + yy, cx - widthAt, cx + widthAt, (t + noise(cx,y,4)) % 3 ? tint : colors.pine1);
    }
  }
};
for (let x = -20; x < width + 30; x += 17 + Math.floor(random() * 14)) {
  const top = hillLine(Math.max(0,Math.min(width-1,x)));
  pine(x, top + 57, 35 + Math.floor(random() * 75), random() < .5 ? colors.pine0 : colors.pine1);
}
// Lower lake, with a shifting horizon and layered horizontal color bands.
const lakeStart = (x) => 782 + Math.round(11 * Math.sin(x / 233) + 8 * Math.sin(x / 91));
for (let x = 0; x < width; x++) {
  for (let y = lakeStart(x); y < height; y++) {
    const d = y - lakeStart(x);
    let c = d < 80 ? colors.water0 : d < 235 ? colors.water1 : d < 330 ? colors.water2 : colors.water3;
    if (noise(x,y,31) === 0) c = d < 330 ? colors.waterDim : colors.water1;
    paint(x,y,c);
  }
}
// Long broken reflections draw the sun toward the viewer without soft gradients.
for (let y = 786; y < 1118; y += 5 + Math.floor(random() * 8)) {
  const d = y - 786, half = 12 + Math.floor(d * .52), center = sunX + Math.round(Math.sin(y / 43) * 24);
  const pieces = 3 + Math.floor(random() * 9);
  for (let p = 0; p < pieces; p++) {
    const x = center - half + Math.floor(random() * half * 2);
    const w = 4 + Math.floor(random() * Math.min(88, 12 + d * .2));
    span(y, x, Math.min(center + half, x + w), p % 3 ? colors.reflect1 : colors.reflect0);
  }
}
// Wind ripples repeat in small, stepped clusters across the lake.
for (let i = 0; i < 6200; i++) {
  const x = Math.floor(random() * width), y = 800 + Math.floor(random() * 340);
  if (y < lakeStart(x) + 10) continue;
  const w = 2 + Math.floor(random() * (y < 900 ? 16 : 38));
  const c = i % 9 === 0 ? colors.waterHi : i % 3 === 0 ? colors.waterDim : colors.water0;
  span(y, x, x + w, c);
  if (i % 8 === 0) span(y + 1, x + 3, x + w - 2, colors.water2);
}
// A near-black pixel shore, reeds, stones, and reflected pines close the foreground.
for (let y = 1042; y < height; y++) {
  const edge = 28 + Math.floor((y - 1042) * .27);
  span(y, 0, edge + 42 * Math.sin(y / 21), colors.shore);
  span(y, width - edge - 54 * Math.sin(y / 19), width - 1, colors.shore);
}
for (let i = 0; i < 140; i++) {
  const x = Math.floor(random() * width), y = 1000 + Math.floor(random() * 152);
  if (x < 310 || x > width - 310) {
    const h = 8 + Math.floor(random() * 38), lean = Math.floor(random() * 19) - 9;
    for (let dy = 0; dy < h; dy += 2) paint(x + Math.round(lean * dy / h), y - dy, i % 3 ? colors.reed : colors.pine1);
  }
}
for (let i = 0; i < 80; i++) {
  const x = Math.floor(random() * width), y = 1090 + Math.floor(random() * 55), r = 3 + Math.floor(random() * 13);
  for (let dy = -r; dy <= r; dy += 2) span(y + dy, x - r + Math.abs(dy), x + r - Math.abs(dy), i % 2 ? colors.rock : colors.pine1);
}

const document = {
  version: 1, name: 'Twilight lake beneath the alpine peaks', width, height, loop: false,
  layers: [{ id: 'landscape', name: '2048 × 1152 hand-drawn pixel landscape', visible: true, opacity: 1 }],
  frames: [{ durationMs: 1000, layers: [{ layerId: 'landscape', pixels }] }]
};
validatePixelArt(document);
await mkdir(out, { recursive: true });
await writeFile(resolve(out, 'landscape.json'), `${JSON.stringify(document)}\n`, 'utf8');
const glints = [[1290, 827], [1432, 866], [1360, 903], [1524, 949], [1318, 991], [1480, 1034]];
for (let frame = 0; frame < 4; frame++) {
  const changed = [];
  for (let i = 0; i < glints.length; i++) {
    const [x, y] = glints[i];
    const xx = x + frame * (i % 2 ? 9 : -7);
    changed.push([xx, y, pixels[y][xx]]);
    paint(xx, y, frame % 2 ? colors.reflect0 : colors.waterHi);
  }
  const svg = renderFrameToSvg(document, { pixelSize: 1, title: '2048 by 1152 OpenPixel alpine lake landscape' });
  await writeFile(resolve(out, `frame-${frame + 1}.svg`), svg, 'utf8');
  for (const [x, y, color] of changed) paint(x, y, color);
}
console.log(`Validated native ${width}×${height} OpenPixel pixel matrix and rendered four ${width}×${height} SVG frames.`);
