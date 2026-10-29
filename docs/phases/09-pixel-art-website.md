# Phase 09 — Pixel-art website

## Completed

- Replaced the earlier multi-section page with a full-screen pixel-art landscape, generated as layered OpenPixel JSON and rendered to SVG by `@openpixel/renderer`.
- Added four looping frames with drifting clouds, water glints, and fireflies; clicking the landscape advances the scene.
- Centered the bitmap-rendered OpenPixel title and one-line `npm install @openpixel/renderer` command with a copy control.
- Added the requested bottom bar with “By Yasir Eymen” and the GitHub repository link.
- Kept the layout static-host friendly and responsive, with reduced-motion support and accessible text labels.

## Verification

- `npm run build:www` — renderer and landscape generation succeeded; the OpenPixel schema validator accepted the 96×54, 8-layer, 4-frame artwork.
- Frontend design lint — 0 failures, 0 warnings.
- `node --check www/app.js` and `node --check www/build.mjs` — passed.
- `git diff --check` — passed.
- Browser preview was unavailable in this environment: local HTTP connections timed out, and the browser policy rejected local-file previews. Static build, asset generation, and source checks completed.

## Next

- Publish the `www/` directory through the repository's static hosting configuration when requested.
