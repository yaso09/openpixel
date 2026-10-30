# OpenPixel

OpenPixel, piksel matrisi katmanlarından oluşan resim ve animasyonları oluşturmak, düzenlemek ve görüntülemek için bir araç takımıdır. TypeScript paketi JSON varlıklarını tarayıcıda keskin SVG olarak çizer; Rust CLI ve MCP sunucusu aynı biçimde varlıklar üretir ve düzenler.

## Paketler

- [`@openpixel/renderer`](packages/pixelart/README.md): TypeScript türleri, doğrulama, SVG kare çizimi ve tarayıcı animasyon oynatıcısı.
- [`openpixel`](apps/cli/README.md): Rust komut satırı aracı; SVG dışa aktarımı ve MP4 video kodlama (ffmpeg + libx264 gerekir).
- [`openpixel-mcp`](apps/mcp/README.md): stdio ve Streamable HTTP/SSE üzerinden Rust MCP sunucusu; MP4 video dışa aktarımı da sunar.
- [OpenPixel Pixel Art becerisi](skills/openpixel/SKILL.md): OpenPixel kullanan ajanlar için kaynaklı çizim ve animasyon yönergeleri.

## GitHub üzerinden kurulum

```sh
npm i -g github:yaso09/openpixel
```

TypeScript renderer GitHub deposundan dağıtılır; npm'e yayımlanmayacaktır.

## CLI ve MCP sunucusunu kurma

Linux x86_64 veya Apple Silicon Mac'te:

```sh
curl -fsSL https://raw.githubusercontent.com/yaso09/openpixel/master/install.sh | bash
```

Windows x64 PowerShell'de:

```powershell
irm https://raw.githubusercontent.com/yaso09/openpixel/master/install.ps1 | iex
```

İki kurucu da en son GitHub Release'i indirerek `openpixel` ve `openpixel-mcp` araçlarını kurar. Bash kurucusu varsayılan olarak `~/.local/bin` dizinini kullanır; değiştirmek için `OPENPIXEL_INSTALL_DIR` ayarlayın. PowerShell kurucusu `%LOCALAPPDATA%\OpenPixel\bin` dizinine kurar ve kullanıcı PATH'ine ekler.

### OpenPixel ajan becerisi

Piksel çizim becerisini coding agent'ınıza [Skills CLI](https://skills.sh/) ile kurun:

```sh
npx skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art
```

Node.js/npm yüklüyse platform kurucuları bu beceriyi de global olarak kurar.

### Ajanlara MCP bağlantısı

OpenPixel kurulduktan sonra aşağıdaki komutlardan birini çalıştırarak MCP sunucusunu Codex'e veya Claude Desktop'a otomatik ekleyin; ardından uygulamayı yeniden başlatın:

```sh
openpixel-mcp add codex
openpixel-mcp add claude
```

Başka bir MCP uyumlu ajan için stdio sunucusu ekleyin: komut `openpixel-mcp`, argüman `stdio`. Dosya konumları ve Claude Desktop için elle yapılandırma örneği [MCP kurulum kılavuzunda](apps/mcp/README.md) bulunur.

Depo geliştirmesi için kök dizinde:

```sh
npm install
npm run build
```

Rust araçlarını kök dizinden derleyin:

```sh
cargo build --release --workspace
```

## SVG karesi çizme

```ts
import { renderToSvg, type PixelArtDocument } from "@openpixel/renderer";

const asset: PixelArtDocument = {
  version: 1,
  name: "Tek piksel",
  width: 1,
  height: 1,
  layers: [{ id: "ink", name: "Mürekkep" }],
  frames: [{
    durationMs: 100,
    layers: [{ layerId: "ink", pixels: [["#F04438"]] }]
  }]
};

document.querySelector("#art")!.innerHTML = renderToSvg(asset, {
  pixelSize: 16,
  title: "Kırmızı piksel"
});
```

`mountPixelArt(element, asset)` tarayıcı animasyon oynatmasını sağlar. Oynatıcı `play()`, `pause()`, `seek(frameIndex)` ve `dispose()` metotlarını sunar. JSON biçimi ve API ayrıntıları için [renderer kılavuzuna](packages/pixelart/README.md) ve [v1 şemasına](schemas/pixel-art.v1.schema.json) bakın.

## CLI ile varlık oluşturma

```sh
openpixel new art/yaratik.json --width 16 --height 16 --name Creature --layers body:Body,details:Details
openpixel set-pixel art/yaratik.json --output art/yaratik-painted.json --frame 0 --layer body --x 7 --y 6 --color '#E96B55'
openpixel render art/yaratik-painted.json --scale 8 --output out/yaratik.svg
```

## Proje durumu

OpenPixel geliştirme aşamasındadır. [Yol haritası](docs/ROADMAP.md) ve [aşama raporları](docs/phases/) için ilgili belgelere bakın.

## GitHub otomasyonları

OpenCode; bakımcı komutlarını, salt okunur issue/PR incelemesini ve issue ile düzeltme PR'ı açabilen günlük depo taramasını yürütür. Tag yayınlarında Groq'un ücretsiz planındaki `openai/gpt-oss-120b`, yalnızca uygulama ve paket değişikliklerine dayanarak İngilizce sürüm notları ve `CHANGELOG.md` girdileri üretir; dokümantasyon, workflow ve depo bakımı değişiklikleri dışarıda bırakılır. Aynı notlar GitHub Release'te kullanılır. Ücret oluşmaması için Groq hesabı ücretsiz planda kalmalıdır. Dependabot npm, Cargo ve GitHub Actions bağımlılıklarını haftalık kontrol eder. Ayrıntılar için [otomasyon kurulumuna](docs/automation.md) bakın.

## Katkı

İnsan ve yapay zekâ destekli katkılar açıktır. Değişiklik yapmadan önce [CONTRIBUTING.md](CONTRIBUTING.md) dosyasını okuyun.

## Lisans

MIT. Ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.
