# Aşama 3 — Rust CLI

## Tamamlananlar

- Kök Cargo workspace ve `openpixel-cli` binary crate'i eklendi.
- CLI komutları: `new`, `validate`, `inspect`, `render`, `render-frames`, `set-pixel`, `set-layer`, `add-frame`, `set-frame` ve `set-loop`.
- Düzenleme komutları kaynak dosyayı otomatik ezmez; ayrı `--output` hedefi ister.
- SVG dışa aktarımı TypeScript renderer ile ortak özellikleri korur: kare seçimi, ölçek, arka plan, başlık, layer sırası/görünürlüğü/opaklığı ve keskin piksel kenarları.
- Palette/frame süreleri ve loop bilgisi `inspect` ile okunabilir. Yeni asset üretimi şeffaf, ölçüleri sabit katman/frame matrisleri oluşturur.
- CLI kullanım ve özellik eşlemesi `apps/cli/README.md` içine yazıldı.

## Renderer ile özellik eşlemesi

| Kütüphane yeteneği | CLI karşılığı |
|---|---|
| JSON v1 doğrulama | `validate` (diğer komutlarda da otomatik doğrulama) |
| İstenen kareyi SVG çizme | `render --frame` |
| Ölçek, arka plan, başlık | `render --scale --background --title` |
| Katman görünürlüğü/opaklığı | JSON alanları ve `set-layer` |
| Animasyon kare ve zamanlaması | `add-frame`, `set-frame`, `render-frames`, `inspect` |
| Döngü bilgisi | `inspect`, `set-loop` |
| Tarayıcı oynatıcı kontrolleri | DOM gerektirdiği için CLI'da DOM kontrolü yok; kare seçimi, sıralı SVG dışa aktarımı ve kare başına durum çıktısı sunulur |

## Doğrulama

- `cargo fmt --all` ve `cargo check --workspace --offline` başarılı.
- Gerçek CLI akışıyla yeni asset, piksel düzenleme, frame ekleme/süresi, layer düzenleme, loop ayarı, doğrulama, inceleme, tek kare render ve tüm kareleri dışa aktarma denendi.
- Üretilen SVG'de boyut, ölçek, renk, başlık kaçışlaması ve kare sayısı kontrol edildi.
- Otomatik test dosyası eklenmedi veya çalıştırılmadı.

## Sonraki aşama

Aşama 4'te bu CLI'nin aynı işlevlerini MCP araçları olarak sunacağım. MCP stdio ve SSE taşıma seçeneklerini, SDK/taşıma kütüphanelerinin kullanılabilirliğini doğrulayıp raporlayacağım.
