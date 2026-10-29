const glyphs = {
  A:'01110100011000111111100011000110001', B:'11110100011000111110100011000111110', C:'01111100001000010000100001000001111', D:'11110100011000110001100011000111110', E:'11111100001000011110100001000011111', F:'11111100001000011110100001000010000', G:'01111100001000010111100011000101110', H:'10001100011000111111100011000110001', I:'11111001000010000100001000010011111', J:'00111000100001000010100101001001100', K:'10001100101010011000101001001010001', L:'10000100001000010000100001000011111', M:'10001110111010110101100011000110001', N:'10001110011010110011100011000110001', O:'01110100011000110001100011000101110', P:'11110100011000111110100001000010000', Q:'01110100011000110001101011001001101', R:'11110100011000111110101001001010001', S:'01111100001000001110000010000111110', T:'11111001000010000100001000010000100', U:'10001100011000110001100011000101110', V:'10001100011000110001100010101000100', W:'10001100011000110101101011101110001', X:'10001100010101000100010101000110001', Y:'10001100010101000100001000010000100', Z:'11111000010001000100010001000011111',
  '0':'01110100011001110101110011000101110', '1':'00100011000010000100001000010001110', '2':'01110100010000100010001000100011111', '3':'11110000010000101110000010000111110', '4':'00010001100101010010111110001000010', '5':'11111100001000011110000010000111110', '6':'01110100001000011110100011000101110', '7':'11111000010001000100010000100001000', '8':'01110100011000101110100011000101110', '9':'01110100011000101111000010000101110',
  '.':'00000000000000000000000000000110000', ',':'00000000000000000000000000100010000', ':':'00000011000011000000000000110000110', '!':'00100001000010000100001000000000100', '?':'01110100010000100010001000000000100', '-':'00000000000000011111000000000000000', '+':'00000001000010011111001000010000000', '/':'00001000100001000100010001000010000', "'":'00100001000000000000000000000000000', '"':'01010010100000000000000000000000000', '#':'01010111110101011111010100101000000', '=':'00000000001111100000111110000000000', '@':'01110100011011110101101111000001111', ' ':'00000000000000000000000000000000000', '↑':'00100011100101000100010000100001000', '↗':'00001000110010110001010010100101000010', '↓':'00100001000010010101011100010000000', '◀':'00100011001111101100001000000000000', '▶':'00100001101111100110001000000000000', '*':'00000010101010100100010101010000000'
};
for (const key of Object.keys(glyphs)) if (glyphs[key].length !== 35) console.warn(`Bitmap glyph ${key} has an invalid pattern`);
const pixelText = [...document.querySelectorAll('[data-pixel-text]')];
const originalText = new WeakMap(pixelText.map((element) => [element, element.textContent.trim()]));
function drawBitmap(element, value) {
  const text = (value ?? originalText.get(element) ?? '').toLocaleUpperCase('en-US');
  const scale = Number(element.dataset.pixelSize || 2);
  const width = Math.max(1, text.length * 6 - 1);
  const rects = [];
  for (let i = 0; i < text.length; i++) {
    const pattern = glyphs[text[i]] || glyphs['*'];
    for (let y = 0; y < 7; y++) for (let x = 0; x < 5; x++) {
      if (pattern[y * 5 + x] === '1') rects.push(`<rect x="${i * 6 + x}" y="${y}" width="1" height="1"/>`);
    }
  }
  element.innerHTML = `<svg aria-hidden="true" viewBox="0 0 ${width} 7" width="${width * scale}" height="${7 * scale}" fill="currentColor" xmlns="http://www.w3.org/2000/svg">${rects.join('')}</svg>`;
}
pixelText.forEach((element) => drawBitmap(element));

const scene = document.querySelector('#landscape');
const landscape = document.querySelector('#landscape-frame');
const motion = window.matchMedia('(prefers-reduced-motion: reduce)');
let frame = 0, timer;
function showFrame(next) { frame = (next + 4) % 4; landscape.src = `./assets/landscape/frame-${frame + 1}.svg`; }
function animate() { clearTimeout(timer); if (!motion.matches) timer = setTimeout(() => { showFrame(frame + 1); animate(); }, [900,700,1000,800][frame]); }
scene.addEventListener('click', () => { showFrame(frame + 1); animate(); });
motion.addEventListener('change', animate);
animate();

const copyButton = document.querySelector('#copy-command');
const command = 'npm install @openpixel/renderer';
copyButton.addEventListener('click', async () => {
  try {
    await navigator.clipboard.writeText(command);
    document.querySelector('#copy-status').textContent = 'Installation command copied.';
    copyButton.setAttribute('aria-label', 'Installation command copied');
    drawBitmap(copyButton.querySelector('[data-pixel-text]'), 'COPIED');
    window.setTimeout(() => { copyButton.setAttribute('aria-label', 'Copy installation command'); drawBitmap(copyButton.querySelector('[data-pixel-text]'), 'COPY'); }, 1400);
  } catch { document.querySelector('#copy-status').textContent = 'Clipboard unavailable. Select the command to copy it.'; }
});