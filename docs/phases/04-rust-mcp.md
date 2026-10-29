# Aşama 4 — Rust MCP

## Tamamlananlar

- Resmî Rust MCP SDK'sı `rmcp 3.8.0` ile `openpixel-mcp` binary crate'i eklendi.
- MCP, CLI crate'inin paylaşılan v1 veri modeli, doğrulaması, dosya işlemleri ve SVG renderer'ını kullanıyor.
- On MCP aracı tamamlandı: `create_asset`, `validate_asset`, `inspect_asset`, `render_frame`, `render_all_frames`, `set_pixel`, `set_layer`, `add_frame`, `set_frame_duration`, `set_loop`.
- stdio taşıması varsayılan; `http` modu `/mcp` endpoint'inde Streamable HTTP sunuyor. HTTP yanıtları akış gerektirdiğinde SSE `text/event-stream` biçiminde dönüyor.
- Araç şemaları JSON Schema olarak SDK tarafından yayımlanıyor. Düzenleme işlemleri kaynak ve hedef yolu açıkça alıyor.
- Çalıştırma ve taşıma açıklamaları `apps/mcp/README.md` içine eklendi.

## Taşıma notu

Güncel resmî Rust SDK, Streamable HTTP taşımasını ve bunun içindeki SSE yanıt akışını destekliyor; eski iki endpoint'li 2024 HTTP+SSE taşımasını bilerek sunmuyor. OpenPixel mevcut Streamable HTTP/SSE standardını uygular. 2024-11-05 sürümüne özel eski SSE istemcileriyle uyumluluk sağlanmıyor. Bu karar resmi SDK'nın [taşıma belgesine](https://github.com/modelcontextprotocol/rust-sdk#transports) göre verildi.

## Doğrulama

- `cargo fmt --all` ve `cargo check --workspace --offline` başarılı.
- stdio üzerinden initialize, tools/list (10 araç), create, set_pixel, add_frame, render_all_frames ve validate çağrıları doğrulandı.
- Yerel HTTP endpoint'i initialize ve tools/list isteklerine SSE akış yanıtı verdi; 10 araç listelendi.
- HTTP loopback doğrulaması kısıtlı ortam izni gerektirdi ve izinle tamamlandı.
- Bu aşamada otomatik test dosyası eklenmedi/çalıştırılmadı; taşıma smoke akışları elle doğrulandı.

## Sonraki aşama

Aşama 5'te npm workspaces kurulumu, paketleme/yayın ayarları ve İngilizce ana README ile Türkçe README tamamlanacak. TypeScript bağımlılığı kurulduktan sonra renderer derlemesi de doğrulanacak.
