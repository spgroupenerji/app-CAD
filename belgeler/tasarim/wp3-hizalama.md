# P3 — şerit hizalaması ve PUA glyph düzeltmesi

> **Tarih:** 2026-10-27 · **Durum:** tamamlandı · **İlgili:** [wp2-wp3-dilim.md](wp2-wp3-dilim.md)

## Kök neden 1 — ikon fontu PUA çakışması

Lucide glyph'leri özel kullanım alanındadır (PUA). İkon fontu `Proportional` ailesinin **sonuna**
eklendiğinde, aynı codepoint'i taşıyan sistem fontları (ör. Segoe UI Symbol) önce çözülüyor ve
"Text"/"Multiline Text" düğmelerinde siyah daire içinde yanlış glyph çiziliyordu.

**Düzeltme:** `ikonlar::fontlari_ekle` ikon fontunu `Proportional` ve `Monospace` listelerinin
**başına** ekler. Doğrulama: [metin-ikon-kirp2.png](gorseller/wp4/metin-ikon-kirp2.png) —
"Text"/"Multiline Text" artık Lucide `type` glyph'i.

## Kök neden 2 — ikon/etiket hizası

`ciz_buyuk_etiketli` ikonu sabit `br.top() + 21` konumuna, etiketi `br.bottom() - 4 - g` konumuna
çiziyordu; kompakt (84 px) gövdede bu iki sabit çakışıyor/kaçık görünüyordu.

**Düzeltme:** ikon + 4 px boşluk + etiket tek blok olarak ölçülür ve düğme içinde **dikey ortalanır**;
kompakt gövdede ikon 28 → 24 px. Etiket galley'i düğme merkezine göre yatay ortalanır.

## Kanıt

- [draw-kirp.png](gorseller/wp4/draw-kirp.png) — Line/Polyline/Circle/Arc ikon-etiket hizası
- [block-kirp.png](gorseller/wp4/block-kirp.png) — Create Block/Insert hizası
- [metin-ikon-kirp2.png](gorseller/wp4/metin-ikon-kirp2.png) — Text/MText doğru glyph

## Doğrulama

- 207 lib testi + katalog testi yeşil; clippy 0 uyarı; rustfmt temiz.
- `cargo build -p spcad` (debug) başarılı; `cargo build --release -p spcad` arka planda çalıştırıldı.
