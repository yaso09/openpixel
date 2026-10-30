# Phase 23 — Groq release notes and changelog

## Completed

- Replaced OpenCode CLI release-note generation with Groq's Chat Completions API, following the supplied release workflow pattern.
- Selected the currently supported `openai/gpt-oss-120b` model; the example's `llama-3.3-70b-versatile` model is deprecated. This model is available within Groq's Free plan rate limits; its separate Developer plan is usage-priced.
- Groq generates Turkish Markdown release notes from commit subjects since the previous tag; the workflow writes the same notes into the release body and `CHANGELOG.md`.
- Documented the required `GROQ_API_KEY` Actions secret and updated Turkish release automation descriptions.

## Verification

- Confirmed the model ID and chat completion fields against the official [Groq model catalog](https://console.groq.com/docs/models) and [API reference](https://console.groq.com/docs/api-reference).
- `git diff --check` — passed.
