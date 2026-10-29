# Aşama 6 — Katkı rehberi ve otomasyonlar

## Tamamlananlar

- `CONTRIBUTING.md` geliştirme kurulumu, doğrulama, Conventional Commit, PR beklentileri ve AI destekli katkı sorumluluklarıyla tamamlandı.
- OpenCode komut workflow'u issue yorumlarında ve satır içi PR yorumlarında `/opencode` ve `/oc` tetiklerini destekliyor; OWNER/MEMBER/COLLABORATOR dışındaki kullanıcılar tetikleyemiyor.
- Aynı depodaki taslak olmayan PR'lar için salt okunur otomatik inceleme workflow'u eklendi.
- Yeni, yetkili katkıcı issue'ları için salt okunur triage; haftalık ve elle tetiklenebilir issue/PR bakım raporu eklendi.
- npm, Cargo ve GitHub Actions için haftalık Dependabot güncelleme kaynakları tanımlandı.
- GitHub Action referansları tam commit SHA'larına sabitlendi. Dependabot Actions ekosistemi bu referansları güncel tutacak.
- `docs/automation.md` secret kurulumu, token izinleri, olay filtreleri ve workflow davranışlarını belgeliyor.

## Güvenlik ve izinler

- Workflow'lar `GITHUB_TOKEN` için job bazında en az gerekli kapsamı belirtir; güncel OpenCode `use_github_token` modu kullanıldığı için OIDC veya OpenCode App token değişimi gerekmez.
- Kod değişikliği talebi yapan yorum workflow'u yalnızca yetkili katkıcıdan gelen tetiklerde `contents`, `issues` ve `pull-requests` yazma iznine sahip.
- PR inceleme ve issue triage workflow'larında OpenCode `bash` ve `edit` izinleri reddedilir. PR inceleme aynı repo dalıyla sınırlıdır; fork PR kodu checkout edilmez.
- Haftalık bakım raporunun GitHub token'ı salt okunurdur ve workflow dosya/komut araçlarını kapatır.
- OpenCode Go sağlayıcı anahtarı `OPENCODE_API_KEY` Actions secret olarak tanımlanmalıdır; varsayılan model `opencode-go/gpt-5.6-luna` olarak ayarlanmıştır. Bu depodan dış servise çağrı veya yayın yapılmadı.

## Doğrulama

- PyYAML ile dört workflow dosyası ve Dependabot yapılandırması parse edildi.
- Workflow action referanslarının her birinin 40 karakter tam SHA olduğu doğrulandı.
- Salt okunur workflow'ların `OPENCODE_PERMISSION` JSON yapılandırması parse edildi ve bash/edit reddi kontrol edildi.
- GitHub Actions çalışma zamanı bu yerel ortamda çalıştırılamaz. Secret, Actions ayarı ve GitHub App/token izinleri repo ayarlarında yapılandırılmalıdır; belgede gereken adımlar yer alıyor.

## Sonraki aşama

Aşama 7'de son entegrasyon yapılacak: TypeScript/Cargo kontrollerini birlikte çalıştırıp örnek varlığın CLI, MCP ve renderer arasında uyumunu doğrulayacağım; sürüm ve yayın öncesi yönergeleri tamamlayacağım.
