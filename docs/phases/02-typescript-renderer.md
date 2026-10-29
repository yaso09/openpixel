# Aşama 2 — Veri biçimi ve TypeScript renderer

## Tamamlananlar

- `schemas/pixel-art.v1.schema.json` ile OpenPixel v1 JSON sözleşmesi tanımlandı.
- `packages/pixelart` içine tipler, çalışma anı doğrulayıcısı, SVG renderer ve tarayıcı animasyon oynatıcısı eklendi.
- Statik kare render'ı; çözünürlük ölçeği, görünürlük, opaklık, şeffaf pikseller, arka plan ve erişilebilir SVG başlığı destekliyor.
- Animasyon oynatıcısı otomatik oynatma, duraklatma, kareye gitme, kare bildirimi ve dispose sunuyor.
- İki kareli JSON örneği eklendi.

## Veri biçimi kararları

- `version` şu an `1`; belge `width`/`height`, sıralı layer metaverisi ve sıralı frame'lerden oluşur.
- Her frame, her layer için tam boyutlu `pixels` matrisi taşır. `null` şeffaf piksel; renk `#RRGGBB` veya `#RRGGBBAA` biçimindedir.
- Kare süresi milisaniyedir; layer sırası arkadan öne çizim sırasını belirler.
- Bozuk ölçüler, yinelenen/eksik layer kimlikleri ve hatalı renkler reddedilir.

## Doğrulama

- JSON Schema ve örnek dosya Node.js ile parse edilerek sözdizimi doğrulandı.
- TypeScript derlemesi çalıştırılamadı; ortamda `tsc` kurulu değil ve paket bağımlılıkları henüz kurulmadı.
- Bu aşamada test dosyası eklenmedi/çalıştırılmadı.

## Sonraki aşama

Aşama 3'te Rust CLI geliştirilecek. CLI, aynı v1 belgesini okuyup doğrulama, kare dışa aktarma ve animasyon/asset oluşturmayı kolaylaştıran komutlar sunacak. CLI özellikleri ve renderer API'si arasında komut-komut eşleme raporda yer alacak.
