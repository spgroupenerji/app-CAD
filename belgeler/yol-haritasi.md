# SPCAD yol haritası ayrıntısı

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-12 · **Değişiklik:** küçük (statik bloklar için Blok Düzenleyici: "Blokları kullan" kanıtı güncellendi) · **Hedef:** Autodesk AutoCAD 2027

İleriye dönük plan: durumlu kilometre taşları, mevcut odak ve tahminlerle sıradaki adımlar. Özet
[YOL-HARITASI.md](../YOL-HARITASI.md); her satırın arkasındaki iş öğeleri [eksikler.md](eksikler.md); sayılar
[hedef-uygulama-paritesi.md](hedef-uygulama-paritesi.md)'den. Saatler Opus 5.5 ajan-saatidir.

## Mevcut odak

1. **Kapıyı kur ve birleştir.** (`cargo fmt` main'de düzeltildi.) `cargo xtask ci` çalıştıran bir
   PR iş akışı ekleyin, sonra 47 açık PR'ı inceleyip birleştirin; çoğu topluluk hata düzeltmesi
   (S1–S3). ≈ 15–25 sa.
2. **Kaydetmede veri kaybını durdurun** (FF1, FF2, FF7) ve DWG/DXF'i seçilen sürümde kaydedin (FF3).
   ≈ 30–45 sa.
3. **Gerçek dosya korpusu** ve çıktımızın AutoCAD'de kontrolü (FF5, sahibi gerektirir).
4. **Hassas girdi:** nesne yakalama izleme, uzantı/paralel yakalamalar, FROM/M2P/TK, yazılan yakalama
   geçersiz kılmaları, düzenlenebilir dinamik girdi (U1–U4). ≈ 25–35 sa.
5. **En çok kullanılan komutların istem seçeneklerini taslaktan çıkarın** (F1). ≈ 15–25 sa.
6. **Otomatik kaydetme, kurtarma, AUDIT, RECOVER** (F4). ≈ 10–15 sa.
7. **Alfa kapısının geri kalanı:** blok düzenleyici ve REFEDIT, dış referans (xref) ekleme (F2, F3) ve çizim
   iletişim kutusu ve önizlemeyle çizim stilleri (F14). ≈ 50–80 sa.

2–5 ve 7. maddeler [alfa kapısı](#alfa-geçidi) satırlarıdır.

## Alfa geçidi

Tipik bir AutoCAD profesyonelinin her gün yaptığı temel iş akışları (craftrules progress-docs
standardı). SPCAD **alfa öncesi**dir çünkü beş iş akışından dördü kapıdan geçemiyor; başarısız
satırlar alfa kontrol listesidir. Ana platform: macOS.

| Temel iş akışı | Uçtan uca çalışıyor mu? | Kanıt | Geçmek için saat |
|---|---|---|---|
| Nesne yakalamaları ve izlemeyle hassas çizim ve düzenleme | **kısmi, engelleyici** | Çalışan yakalamalar, kutupsal, orto ve koordinat girişi çalışıyor; nesne yakalama izleme, uzantı ve paralel yakalamalar davranışsız anahtarlar; FROM/M2P/TK veya yazılan yakalama geçersiz kılmaları yok; TRIM/EXTEND standart modu, OFFSET Erase/Layer/Multiple, ROTATE Reference "not available yet" yazıyor; elips/spline kesişimleri yaklaşık ([arayuz-paritesi.md](arayuz-paritesi.md), [eksikler.md](eksikler.md) U1, U3, U4, F1) | 35–50 |
| Ölçülendirme ve notlandırma | kısmi, engelleyici değil | Tüm temel DIM türleri, ilişkisellik, DIMSTYLE değişkenleri (~80'in 43'ü), TEXT/MTEXT, MLEADER, tablolar çalışıyor; DIMSTYLE iletişim kutusu yok, MTEXT düzenleyici yerinde değil, MLEADER zayıf (F7, F9, F10). Bir teknik ressam bir sayfayı uçtan uca ölçülendirebilir | Kapı için 0 (beta işi) |
| Blokları kullanma (kütüphane bloklarını ekleme, düzenleme, nitelikler, xref'ler) | **kısmi, engelleyici** | Statik BLOCK/INSERT/nitelikler, ATTSYNC, BATTMAN düzenleme, xref'ler (#525) ve Blok Düzenleyici (BEDIT/BSAVE/BCLOSE: her başvuru güncellenir) çalışıyor; REFEDIT yok, bu yüzden bir blok çizim içinde yerinde düzenlenemiyor; XCLIP yok; diğer çizimlerden gelen dinamik bloklar davranışlarını kaybediyor (F2, F3) | 35–55 |
| Katmanlar, yerleşimler ve bir sayfayı çizdirme | **kısmi, engelleyici** | Katmanlar, yerleşimler, görünüm pencereleri, sayfa kurulumları, PDF çıktısı, çizim stilleri (CTB/STB, #410), çizdirme iletişim kutusu (#160), çizdirme önizlemesi ve PUBLISH (#186) ve çizici aygıtları (#564) çalışıyor; yazıcılar yalnızca PDF/PNG/SVG dosyaları üzerinden (F14, H3) | 15–25 |
| AutoCAD'in kayıpsız açtığı DWG'yi kaydetme ve başkalarının DWG'lerini yeniden açma | **hayır** | DXF modellemediğimiz şeyleri koruyor (#528) ve yerel MULTILEADER'ları yazıyor (#459, henüz DWG üzerinden test edilmedi); DWG üzerinden acadrust özel sınıf varlık ve nesneleri ve bazı yabancı verileri düşürüyor; AutoCAD'de hiç kontrol edilmedi; gerçek dosya testi yok; 10 MB'lık bir DWG ~2 dk sonra başarısız oldu (#57) (FF1–FF6) | 40–60 + sahip (gerçek dosya korpusu, AutoCAD kontrolü) |
| **Alfa'ya** | | | **≈ 125–190** |

Gerçek işe hazır ≈ 38%, bu da ~40% alfa çubuğunun altında; kapı satırlarını kapatmak onu 40%'ın
üzerine taşır.

## Beta kontrol listesi

Beta şu anlama gelir: ana akım 2B çalışma için özellik açısından neredeyse tamamlanmış, DWG'yi
güvenilir şekilde açıp kaydeder, kalan eksikler bilinen hatalar ve uç özelliklerdir (gerçek işe
hazır ≈ 75%).

| Öğe | Durum | Tah. |
|---|---|---|
| DWG/DXF: kaydetmede hiçbir şey kaybolmaz; yerel MULTILEADER; sürüm seçimi; kod sayfaları; gerçek dosya korpusu yeşil; AutoCAD'de doğrulandı | başlamadı | 50–75 |
| Elipsler ve spline'lar için tam 2B geometri; poliline ofseti doğru; göreli toleranslar | başlamadı | 50–80 |
| AutoCAD düzeyinde hassas girdi (izleme, dinamik girdi, geçersiz kılmalar, nokta değiştiriciler, tutma noktaları, seçim anahtar sözcükleri, bağlam menüleri) | kısmi | 90–140 |
| Her Draw/Modify istem seçeneği çalışır; ilişkisel diziler | kısmi | 40–60 |
| Notlandırma derinliği: DIMSTYLE yöneticisi ve değişkenler, eksik DIM komutları, yerinde MTEXT, alanlar, MLEADER düzenleme, not ölçeklendirme, tablolar | kısmi | 90–130 |
| Xref'ler, blok düzenleyici, dinamik bloklar, gruplar, nitelik araçları | başlamadı | 90–135 |
| Çizim stilleri, çizim önizlemesi, PUBLISH, dikdörtgen olmayan görünüm pencereleri | kısmi | 30–45 |
| Otomatik kaydetme, kurtarma, AUDIT/RECOVER, Options iletişim kutusu, şablonlar, SHX, kullanıcı `.lin`/`.pat` | kısmi | 40–60 |
| AutoLISP çekirdek alt kümesi ve SCRIPT | tamam (AutoLISP #758, SCRIPT #693); Visual LISP, DCL ve reactor'lar gelecek | 30–50 |
| Katalog artı Mac için AutoCAD'in dilleri | başlamadı | 30–45 |
| Kararlılık: CI kapısı, sorun birikimi, komut duman taraması, geometri fuzz testi | kısmi | 30–50 |
| Platformlar: web açma/kaydetme, Windows/Linux çalışma zamanı kontrolleri; 1M varlıkta ve büyük DWG'de performans | kısmi | 20–35 |
| **Betaya toplam** | | **≈ 600–930** |

Bunun yaklaşık 70%'i 4–6 ajan arasında paralelleşir (satır başına bir ajan), bu yüzden geçen süre
kabaca 150–250 saattir.

## Kilometre taşları

Durum 2026-10-10'da varlığa göre değil derinliğe göre yeniden puanlandı (her kilometre taşını inşa
eden ajanların bildirdiği 2026-10-07 yüzdeleri [YOL-HARITASI.md](../YOL-HARITASI.md)'nin ilerleme günlüğünde).

| # | Kilometre taşı | Durum | Kalan (sa) |
|---|---|---|---|
| M0 | İskelet ve dikey dilim: çalışma alanı, AutoCAD tarzı arayüz, komut satırı, çizim/düzenleme temelleri, DXF, CLI, MCP, web | tamam | — |
| M1 | Çizim çekirdeği: her 2B Draw komutu ve seçeneği, dinamik girdi, nesne yakalama izleme, yakalama geçersiz kılmaları | ≈ 55% (Draw genişliği güçlü; izleme, dinamik girdi, geçersiz kılmalar eksik) | 35–55 |
| M2 | Düzenleme: tutma noktaları, PEDIT, SPLINEDIT, LENGTHEN, BLEND, ALIGN, ilişkisel diziler, tüm eğriler için trim/extend, MATCHPROP arayüzü | ≈ 50% | 45–70 |
| M3 | Katmanlar ve özellikler | ≈ 70% (açık katman hataları #71–#75, #87) | 10–15 |
| M4 | Notlandırma: DIM*, DIMSTYLE yöneticisi, MLEADER, yerinde MTEXT, alanlar, tablolar, not ölçeklendirme, SHX | ≈ 40% | 90–130 |
| M5 | Tarama ve bloklar: tarama derinliği, blok düzenleyici, dinamik bloklar, xref'ler, gruplar, nitelikler | ≈ 30% | 105–160 |
| M6 | Yerleşimler ve çizdirme: görünüm pencereleri, sayfa kurulumları, çizim stilleri, önizleme, PUBLISH | ≈ 35% | 35–50 |
| M7 | Dosyalar: DWG/DXF sadakati ve sürümleri, RECOVER/AUDIT, şablonlar, otomatik kaydetme, ETRANSMIT, altlıklar | ≈ 30% | 170–280 (diğer biçimler dahil) |
| M8 | Sorgu ve yardımcılar, OPTIONS, özelleştirme | ≈ 25% | 45–70 |
| M9 | Parametrik | ≈ 50% | 20–30 |
| M10 | Performans: 60 fps'te 1M varlık, büyük DWG yüklemeleri | ≈ 60% (yalnızca iç sayılar) | 20–40 |
| M11 | 3B: UCS, yörünge, görsel stiller, katılar, yüzeyler, ağlar, kesitler, işleme | ≈ 2% | 300–450 |
| M12 | Otomasyon: AutoLISP, SCRIPT, eylem kaydedici, sayfa kümeleri, Express Tools | ≈ 15% (MCP, CLI, kontrol kanalı tamam) | 130–220 |
| M13 | 1.0 cilası: tercihler, çalışma alanları, yerelleştirme, erişilebilirlik, imzalı paketler, belgeler | ≈ 15% (her platform için imzalı paketler var; temalar tamam; Ukraynaca ve Almanca dize kataloğu, canlı dil değiştirme, #36) | 110–170 |
| — | Geometri hassasiyeti (boyutlar arası, bkz. [geometri-paritesi.md](geometri-paritesi.md)) | ≈ 40% | 70–110 |
| — | Yapay zekâ özellikleri | uygulama içinde ≈ 0% | 40–80 |
| | **Tam pariteye kalan** | | **≈ 1,300–2,100** |

Kilometre taşı satırları kesişen satırlarla biraz örtüşür; toplam, sütun toplamı değil
[hedef-uygulama-paritesi.md](hedef-uygulama-paritesi.md) içindeki boyut toplamıdır.

## Beta sonrası

3B temelleri (UCS, 3B görünümler, yörünge, sonra `plan/adr/0001` uyarınca katılar), sayfa kümeleri,
tam AutoLISP yüzeyi, açık lisanslı yerel modellerle yapay zekâ özellikleri ve kalan her dil.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-12 | küçük | Statik bloklar için Blok Düzenleyici geldi: "Blokları kullan" kanıtı güncellendi; durum ve saatler değişmedi (REFEDIT ve dinamik bloklar kalıyor) |
| 2026-10-11 | küçük | Birleştirilen iş için alfa kapısı kanıtı yenilendi (#459, #525, #528, #160, #186, #564); durumlar ve saatler değişmedi |
| 2026-10-10 | küçük | Main birleştirildi: fmt düzeltildi, M13'te Ukraynaca katalog belirtildi, gerçek işe hazır 38% |
| 2026-10-10 | küçük | Alfa kapısı eklendi (5 temel iş akışı, 4'ü başarısız); aşama alfa öncesi |
| 2026-10-10 | büyük | Eski ROADMAP'in kilometre taşı tablosundan ve mevcut odaktan oluşturuldu; kilometre taşları derinliğe göre yeniden puanlandı; beta kontrol listesi eklendi |
