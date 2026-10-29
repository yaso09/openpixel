# OpenPixel — Yol Haritası

Bu belge, projeyi doğrulanabilir aşamalara böler. Her aşama tamamlandığında ilgili `docs/phases/` raporu güncellenir; bir sonraki aşama başlamadan önce o raporda sonuç, doğrulama ve sıradaki işler kayda geçer.

## Hedef

Katman matrislerinden oluşan JSON dosyalarını SVG üzerinde çizen TypeScript kütüphanesi; aynı veri biçimini kullanarak görsel/animasyon üreten Rust CLI ve MCP. MCP stdio yanında SSE taşımasını da destekleyecek. npm üzerinden kolay kurulum, İngilizce ana README ve Türkçe README, katkı rehberi ve GitHub otomasyonları teslim kapsamındadır.

## Aşamalar

| Aşama | Kapsam | Çıkış ölçütü |
|---|---|---|
| 1. Depo ve plan | Git, monorepo düzeni, MIT lisansı, başlangıç belgeleri | Depo başlatılmış, klasörler ve bu yol haritası mevcut; rapor yazılmış |
| 2. Veri biçimi ve TS renderer | JSON şeması, katman/frame modeli, SVG çizimi/animasyonu, örnekler | Belgelenmiş biçim; kütüphane API'si ve doğrulanmış temel senaryolar |
| 3. Rust CLI | Dosya işlemleri, renderer uyumlu asset üretimi/düzenlemesi, kolaylaştırıcı komutlar | CLI komutları kütüphane yetenekleriyle eşleşiyor; yardım ve örnekler |
| 4. Rust MCP | CLI kapsamındaki işlevleri sunan araçlar, stdio ve SSE | Taşıma seçenekleri çalışır; araçlar ve hata davranışı belgeli |
| 5. Paketleme ve kullanıcı belgeleri | npm paketleme/kurulum, İngilizce ve Türkçe README, kullanım örnekleri | Temiz kurulum akışı ve dokümantasyon |
| 6. Katkı ve otomasyon | CONTRIBUTING, yapay zekâ katkısı ilkeleri, GitHub Actions/opencode action iş akışları | PR/issue ve düzenli bakım akışları belgeli ve yapılandırılmış |
| 7. Son entegrasyon | Uçtan uca uyumluluk, sürümleme ve yayın hazırlığı | CLI/MCP JSON çıktısı renderer'da açılıyor; dağıtım talimatları tamam |
| 8. Pixel art becerisi | Güvenilir kaynak ve yapay zekâ çizim ilkelerini OpenPixel JSON/araçlarıyla birleştiren ajan becerisi | Kaynaklı SKILL, OpenPixel iş akışı ve kalite kapısı |

## Tasarım kararları (başlangıç varsayımları)

- Repo kökünde npm workspaces ve Cargo workspace birlikte kullanılacak.
- TypeScript renderer `packages/pixelart` altında, Rust uygulamaları `apps/cli` ve `apps/mcp` altında tutulacak.
- Paylaşılan JSON sözleşmesi `schemas/` altında tanımlanacak; Rust ve TypeScript tarafı aynı sözleşmeye göre doğrulanacak.
- Her aşama için `docs/phases/NN-*.md` raporu tutulacak. Plan değişirse bu dosya güncellenecek.
- İlk sürümlerde SVG render ana hedef; video üretimi için CLI dışa aktarma yaklaşımı ayrı tasarım kararı olarak ele alınacak.

## Şimdiki durum

Aşama 1–8 tamamlandı. Renderer, Rust CLI/MCP, npm paketi, iki dilli belgeler, katkı rehberi, OpenCode Go otomasyonları ve OpenPixel Pixel Art ajan becerisi hazır. Aşama raporları `docs/phases/` altındadır. npm yayını ve GitHub Actions secret kurulumu depo sahibinin yayın yapılandırmasında tamamlanacak.
