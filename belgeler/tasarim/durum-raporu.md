# Tasarım yenilemesi — durum ve kapanış raporu

> **Tarih:** 2026-10-27 · **Dal:** `main` · **Yerel HEAD:** (commit sonrası) · **Uzak HEAD:** `46442f0` (push kimlik doğrulaması bekliyor)

## 1. Tamamlananlar

| Faz | İş | Durum |
|---|---|---|
| **P0** | İkon spike'ı (iconflow/Lucide vs mevcut çizim), 4 ölçekte PNG | ✅ |
| **P1** | `Semantik` sözlüğü (19 rol), WCAG testleri, `ikonlar/` modülü (53 rol), Lucide font kurulumu, durum çubuğu | ✅ |
| **P1** | Token altyapısı: tipografi/boşluk/yarıçap/hareket ölçekleri, `visuals()`/`apply()` | ✅ |
| **P2** | Widget durumları (kenarlık/hover/expansion), pencere gölgesi, palet başlıkları, diyalog iç yüzeyi | ✅ (dilim) |
| **P3** | Durum çubuğu 28 px, komut çubuğu 18/34 px + çerçeve + ipucu, panel/şerit/dosya sekmeleri, ViewCube | ✅ (dilim) |
| **P4** | 141 ikonluk katalog + çizilebilirlik testi; 45 genel ikon Lucide'e bağlı | ✅ (kısmi) |
| **P5** | Token/kontrast testleri, ölçek testleri, Semantik↔Tokens tutarlılık testi, açık/koyu tema görselleri | ✅ (kısmi) |

## 2. Doğrulama

- `cargo test -p spcad-ui-egui --lib`: **207 passed, 0 failed**
- `cargo clippy -p spcad-ui-egui --lib`: 0 uyarı
- `rustfmt --check`: temiz
- `cargo build -p spcad`: başarılı
- Ekran görüntüleri: `belgeler/gorseller/wp0…wp4/`

## 3. Açık kalanlar

| Faz | Kalan | Not |
|---|---|---|
| **P2** | Panel gövde satırları, düğme/alan/tooltip varyantları | `craft-ui` docking çizimi; yerel sarmalayıcı veya upstream rev |
| **P3** | Şerit 84 px kompaktlaştırma (yerleşim algoritması), komut geçmişi paneli | Mevcut sabit yükseklikte etiketler çakışıyor |
| **P4** | 96 CAD ikonunun 24×24 / 1,5 px kuralıyla yeniden çizimi | Önce şeritteki ~20; her partide katalog onayı |
| **P5** | Golden (dark/light × 15 dil × 3 genişlik), a11y denetimi, ham renk allow listesinin kaldırılması | Platformlar arası golden için CI matrisi gerekir |

## 4. Bloke eden iki konu

1. **Push kimlik doğrulaması:** Güvenlik temizliğinden sonra PAT kaldırıldı; kalan GitHub CLI token geçersiz.
   Çözüm: eski PAT'ı revoke et → yeni PAT → `gh auth login -h github.com` → `git push origin main`.
2. **Kalan ikon çizimi ve golden:** Tek turda güvenle tamamlanamaz; her ikon partisi görsel onay ve katalog regenerektirir, golden ise macOS/Linux CI doğrulaması ister.

## 5. Yerelde bekleyen commit'ler (push edilemedi)

`6f9c104` · `d3048ad` · `e14c54e` · `01eee32` · `6262a47` · `5477af7` · `3826f01` · `7210290`

## 6. Sonraki sıra

1. Push kimliğini düzelt → 8 commit'i gönder.
2. P4 ilk 20 CAD ikonunu çiz + katalog onayı.
3. P3 komut geçmişi paneli ve şerit yerleşim algoritması.
4. P5 golden + a11y + allow listesi temizliği.
