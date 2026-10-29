# Aşama 7 — Son entegrasyon ve yayın hazırlığı

## Tamamlananlar

- GitHub OpenCode iş akışlarının sağlayıcısı Anthropic'ten OpenCode Go'ya geçirildi. İş akışları `OPENCODE_API_KEY` secret'ını ve `opencode-go/gpt-5.6-luna` modelini kullanıyor.
- `docs/automation.md` ve Aşama 6 raporu yeni sağlayıcı ayarını açıklıyor. OpenCode Go model biçimi resmi rehberle doğrulandı: `opencode-go/<model-id>`.
- Monorepo entegrasyonu doğrulandı: TypeScript renderer derlendi ve npm paketinin `--dry-run` içeriğinde JS, tipler, şema, README ve MIT lisansı yer aldı.
- CLI, `examples/two-frame-spark.json` dosyasını doğruladı ve iki frame için SVG çıktısı üretti.
- Renderer paketi aynı örnek JSON'u doğruladı ve 2. frame'i ölçekli SVG'ye çizdi.
- Rust workspace biçim kontrolü ve çevrimdışı derlemesi başarılı; MCP binary'si de derlendi.
- Çalışma ağacı ve doküman farkları gözden geçirildi; bu aşamada npm yayını veya uzak servis çağrısı yapılmadı.

## Doğrulama

- `npm run build` — başarılı.
- `npm pack --dry-run --workspace @openpixel/renderer` — başarılı; 14 paket dosyası listelendi.
- `cargo fmt --all -- --check` — başarılı.
- `cargo check --workspace --offline` — başarılı; Windows ortamında incremental cache hard-link uyarıları kopyalamaya geri düştü, derlemeyi etkilemedi.
- `cargo run --offline -p openpixel-cli -- validate examples/two-frame-spark.json` — geçerli OpenPixel v1, 5×5, 1 layer, 2 frame.
- `cargo run --offline -p openpixel-cli -- render-frames examples/two-frame-spark.json --scale 4` — 2 SVG frame oluşturdu.
- Node.js ile kurulu `@openpixel/renderer` paketinden import, doğrulama ve SVG render — başarılı.
- `git diff --check` — başarılı.

## Yayın öncesi gerekenler

- GitHub Actions ayarlarında `OPENCODE_API_KEY` adlı secret'a OpenCode Go anahtarı eklenmeli. Bu depo yerel ortamdan secret yazamaz.
- npm paketi `@openpixel/renderer@0.1.0` olarak yayınlanmaya hazırlandı; registry'ye yayınlanmadı.
- GitHub Actions olay tetikleyicileri ve dış servis çağrıları gerçek depo üzerinde doğrulanmalı.

## Sonraki adım

Planlanan geliştirme aşamaları tamamlandı. Yayın öncesinde Actions secret'ını ekleyip CI iş akışlarını gerçek GitHub deposunda çalıştırın; ardından npm sürümünü yayın kararıyla eşleştirin.
