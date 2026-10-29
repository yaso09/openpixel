# Phase 12 — Minimal landscape landing page

## Completed

- Removed the central wordmark, hint, credits, and repository link from the page.
- Kept the 2048×1152 OpenPixel landscape as the full-screen focus, including its click-through animation.
- Added only the install command and a quiet copy button in a slim bottom bar.

## Verification

- `node --check www/app.js` — passed.
- Frontend design lint — 0 failures, 0 warnings.
- `git diff --check` — passed.

## Next

- Serve the `www/` directory as a static site.
