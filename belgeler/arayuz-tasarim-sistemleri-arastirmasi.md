# Arayüz tasarım araştırması — B askıda, C öneriliyor

> **Tarih:** 2026-10-27 · **Yöntem:** depo kodu taraması, WCAG kontrast ölçümü, crates.io/GitHub birincil kaynakları · **İlgili:** [arayuz-gpui-tasarimi.md](arayuz-gpui-tasarimi.md)

## 1. Dış iddianın denetimi ("wgpu + f16 HDR + Catmull-Rom + egui en optimum")

| İddia | Bu depodaki gerçek | Kanıt |
|---|---|---|
| f16 HDR | **Yok.** 8 bit sRGB yüzey; HDR/f16/Rgba16Float geçmiyor | `grep -rni "hdr|f16|rgba16"` → 12 yanlış pozitif (hepsi `hdr` adlı başlık değişkeni); `*.wgsl` dosyası yok; `gpu.rs` hedef formatı `target.format.is_srgb()` |
| Catmull-Rom | **Yok.** Çizimde kübik B-spline var (`spline_fit.rs`), Catmull-Rom yok | `catmull` için 0 eşleşme |
| wgpu | **Var.** Tuval, `egui_wgpu` paint callback ile kendi wgpu hattını çiziyor; tepe renkleri premultiplied alpha | `arayuz-egui/src/gpu.rs` |
| egui | **Var, 0.36.** egui'nin güncel sürümü **0.37.0** (15 Eki 2026) | Cargo.toml; crates.io |
| SDR/HDR compositing | **Konu dışı.** HDR katmanı olmadığı için birleştirilecek iki renk uzayı yok. Tek gerçek konu sRGB'de alfa/AA karışımı | `gpu.rs:595` |

**Sonuç:** Alıntılanan mimari bu depoya ait değil. Yine de "çatıyı değiştirme, tasarım katmanını güçlendir" sonucu maliyet/fayda açısından savunulabilir — gerekçesi HDR değil, tasarım sistemi borcu.

## 2. `shadcn_egui` gerçeği

| Aday | Sürüm | egui hedefi | Lisans | Popülerlik | Son güncelleme | 0.36 uyumu |
|---|---|---|---|---|---|---|
| `shadcn-egui` (oetiker) | 0.6.0 | **0.34** | MIT | ★1, 110 indirme | 2026-10-24 | ❌ |
| `egui-shadcn` (FerrisMind/shadcn-rs) | 0.5.0 | **0.33** | MIT (crate) / Apache-2.0 (depo) | ★90, 1.700 indirme | 2026-03-11 | ❌ |
| `craft-ui` (mevcut bağımlılık) | — | egui 0.36 | MIT/Apache-2.0 | depo içi | — | ✅ |

- İkisi de **0.36 ile derlenmez**; bağımlılık olarak eklenemez. MIT/Apache-2.0 oldukları için **kaynak kopyalanıp 0.36'ya uyarlanabilir** (shadcn felsefesi zaten budur).
- İkisi de genç: `shadcn-egui` 12 Eki 2026'da yayınlandı; tek kişilik. Üretim bağımlılığı için erken.
- `craft-ui` zaten SVG ikon katalogları, ikon butonlar, sekmeler, panel kontrolleri, docking ve tema yardımcıları sağlıyor; Craft ekosisteminin `docs/theme-infrastructure.md` ve `docs/ui-components-audit.md` belgeleri bu sınırı tanımlıyor.

**Sonuç:** Hazır crate entegrasyonu bugün mümkün değil; en sağlıklı yol **kendi token + bileşen katmanımızı** craft-ui üzerine kurmak, shadcn ilkelerini (semantik renk, varyant, yarıçap, durum katmanları) uyarlamak.

## 3. Mevcut arayüz denetimi (ölçülen)

### Kontrast (WCAG: gövde ≥ 4,5:1; büyük metin/bileşen ≥ 3:1)

| Çift | Oran | Durum |
|---|---:|---|
| panel / chrome | 1,29:1 | Yüzeyler ayrışmıyor |
| control / panel | 1,19:1 | Kontroller panelde kayboluyor |
| canvas / panel | 1,55:1 | Tuval–panel sınırı okunmuyor |
| tab_active / chrome | 1,51:1 | Sekme hiyerarşisi zayıf |
| border / panel | 1,59:1 | 1 px sınır görünmüyor |
| text_faint / panel | 3,27:1 | **AA altı** (gövde metni) |
| beyaz / accent | 3,33:1 | **AA altı** (buton yazısı) |
| metin / accent | 2,63:1 | **AA altı** |
| metin / panel | 8,14:1 | ✅ |

### Tipografi
- Boyutlar dağınık: **9,5 · 10,5 · 11,5 · 12,5 · 13 · 14 · 17 · 18** — ölçek/token yok.
- Şerit etiketleri 9,5–11,5 px: okunabilirlik sınırında; gövde 12,5.
- Font platforma göre değişiyor (Segoe UI / SF / DejaVu); marka fontu yok. Monospace 12,0.
- Satır yüksekliği, harf aralığı, sayı hizalaması tanımlı değil.

### Boşluk ve ritim
- `item_spacing 6/4`, `button_padding 6/3`, `interact_size.y 20`; sabitler 16/18/20/21/26/28/30/40/56/94…
- 4 pt ızgara yok; komşu bileşenlerde farklı padding.

### İkonografi
- 995 satır elle çizim; stroke kalınlığı `self.w`, `2.2` gibi değişken; tek optik grid yok.

### Hareket ve durumlar
- Yalnız egui varsayılan animasyonu + kaydırma; hareket tokenı yok (reduce-motion desteği var).
- Focus ring yalnız ikon butonlarında (1 px); disabled/hover/selected hiyerarşisi zayıf.

## 4. Öneri: C — egui tasarım sistemi yenilemesi

| İş paketi | Çıktı | Süre (ajan-saat) |
|---|---|---|
| WP0 Denetim + yön | Token prototipi, önce/sonra ekran görüntüleri, egui 0.36→0.37 değerlendirmesi | 10–16 |
| WP1 Token + tema | Renk/tipografi/boşluk/yarıçap/gölge/hareket; `Visuals` + özel boyama tek kaynaktan | 24–36 |
| WP2 Bileşen kiti | Buton, input, combo, tab, menü, diyalog, tablo, ağaç, tooltip, toast | 40–60 |
| WP3 Krom yenileme | Başlık+menü, şerit (kompakt), sekmeler, palet başlıkları, komut güvertesi, durum çubuğu, ViewCube | 40–60 |
| WP4 İkonografi | 24 px grid, 1,5 px stroke; Lucide eşlemesi + CAD ikonlarının yeniden çizimi | 20–30 |
| WP5 Erişilebilirlik + kapılar | AA kontrast, focus, golden testler, 15 dil, performans | 20–30 |
| **Toplam** | | **154–232** |

**Tasarım yönü (AutoCAD düzeni korunur):** üç yüzey + overlay; 1 px hairline ve iki kademe gölge; tek aksan; Inter (OFL) + JetBrains Mono (OFL); 4 pt ızgara; satır 24/28/32; yarıçap 4/6/8; hareket 90/150/220 ms; ⌘K komut paleti; komut güvertesi; şerit 124 px yerine kompakt + genişleyen; palet başlıkları 32 px; durum çubuğu gruplu; ViewCube sadeleştirilmiş.

**Renk/kompozisyon:** sRGB'de kalınır; HDR eklenmez. Düzeltilecek gerçek konular: premultiplied alpha tutarlılığı, AA kenarlarında sRGB karışımı (egui sınırı), gerekirse dithering. f16/HDR yalnızca fotoğrafik/bloom içerik için anlamlı; 2B CAD arayüzünde kazanç yok, maliyet yüksek.

## 5. Karar

- **B (GPUI) askıya alınır.** Maliyeti 450–700 ajan-saat; platform kaybı (Android yok, web vitrin), tasarım tavanı C'ye göre sınırlı kazanç.
- **C uygulanır.** 154–232 ajan-saat; 2–3 paralel ajanla ~50–80 duvar saati; mevcut mimari (engine, wgpu tuval, i18n, denetim sözleşmesi) korunur.
- Cutover yok, geri dönüş kolay: tasarım sistemi modülü kademeli devreye alınır, her adımda golden ekran testi.

## 5b. Bağımsız doğrulama (2026-10-27, ikinci ajan)

Tam rapor: [arastirma/egui-tasarim-sistemleri.md](arastirma/egui-tasarim-sistemleri.md) (125 satır, her iddiaya kaynak).

| Ek bulgu | Sonuç |
|---|---|
| Dört adayın egui isteği | shadcn-egui `^0.34`, egui-shadcn `^0.33`, pjankiewicz `0.33` (yayınlanmamış), ouroboros-ui `0.34.1` (arşivlenmiş) → **hiçbiri 0.36 değil**; caret kuralı gereği ikinci egui kopyası ve tip uyuşmazlığı |
| Kimlik çakışmaları | crates.io `egui-shadcn` = FerrisMind (pjankiewicz'in 60+ bileşenli projesi yayınlanmamış); crates.io `shadcn-rs` = hughdbrown (Yew/WASM); yayınlanan shadcn-egui = oetiker |
| craft-ui upstream sürüklenmesi | `main` artık `[patch.crates-io]` ile **fork'lu egui 0.37** (storytold/egui rev 6d518fc) ve **fork'lu winit** kullanıyor; pinli rev 5c874d5 crates.io egui 0.36. Yükseltme = fork kabulü; ayrı karar |
| crates.io adaşı | `craft-ui` 1.4.7 = **aphrody-labs/craft-libs** (15 indirme), bizimki storytold/craft-libs → **sürümle asla eklenmemeli** |
| 0.36 uyumlu ekosistem | egui_extras 0.36.2 (tablo için tek aday), egui_dock gereksiz (craft-ui docking), egui-phosphor gereksiz (craft-ui ikon kataloğu), egui_kittest 0.36.2 zaten 44 referansta |
| Ölçüm | arayuz-egui 94 dosya / 29.545 satır; tema.rs 34,9 KB; i18n 15 katalog; egui 0.36.2 kilitli |

**WP0'a eklenen kararlar:** (1) semantik token sözlüğü (background/foreground/primary/muted/border/ring), (2) `clippy.toml` `disallowed-methods` ile ham `Color32::from_rgb`/boşluk yasağı (tema.rs istisnası), (3) egui 0.37 + fork kararı ayrı iş olarak belgelenir, (4) mevcut `egui_kittest` snapshot'ları bileşen galerisine bağlanır.

## 6. Kaynaklar

- [egui (crates.io, 0.37.0)](https://crates.io/crates/egui) · [shadcn-egui](https://crates.io/crates/shadcn-egui) · [egui-shadcn](https://crates.io/crates/egui-shadcn)
- [oetiker/shadcn-egui](https://github.com/oetiker/shadcn-egui) · [FerrisMind/shadcn-rs](https://github.com/FerrisMind/shadcn-rs)
- [craft-libs (craft-ui)](https://github.com/storytold/craft-libs) · [WCAG 2.2 kontrast](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html)
