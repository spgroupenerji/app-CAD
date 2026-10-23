# Yerelleştirme paritesi

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-14 · **Değişiklik:** küçük (İsveççe katalog o zamandan beri eklenen 95 dizeyle tamamlandı; önceden: İsveççe katalog eklendi: tam katalog, menü başlıkları, ana dili konuşanlarca terim incelemesi; önceden: Hangul yazı tipi kapsamıyla mevcut Korece katalog tamamlandı) · **Hedef:** Autodesk AutoCAD 2027 for Mac (26.0)

SPCAD'nin arayüzü ve çizim metninin AutoCAD'in dillerinden ne kadar uzak olduğu.
[YOL-HARITASI.md](../YOL-HARITASI.md#languages) içinde özetlendi; iş maddeleri
[eksikler.md](eksikler.md#yerelleştirme) içinde.

## Hedefin sundukları

Kurulu AutoCAD 2027 for Mac paketinden yalnızca listeleme yapılarak ölçüldü (klasör adları, hiçbir
şey okunmadı): `Contents/Resources` içinde `Base.lproj` (İngilizce), `de`, `es`, `fr`, `it`, `ja`,
`ko` ve `zh_CN` yerelleştirmeleri var ve `Resources/Support` içinde eşleşen `@de@` … `@zh_CN@`
klasörleri var. Bu, Mac'te **8 arayüz dili** demektir. AutoCAD for Windows ayrıca Geleneksel Çince,
Brezilya Portekizcesi, Rusça, Lehçe, Çekçe ve Macarca için dil paketlerine sahiptir (Autodesk'in
yayımlanmış dil paketi listesi, burada ölçülmedi). AutoCAD'in yazım denetleyicisi yaklaşık 20
sözlükle gelir (`Resources/Support` içindeki `*.dct` adları).

AutoCAD'de çizim metni: TrueType ve SHX büyük yazı tipleri (CJK), Unicode `\U+` kaçışları, karmaşık
yazı betiklerinde şekillendirme ve TrueType aracılığıyla sağdan sola metin, dikey SHX yazı stilleri.

## SPCAD'nin sahip oldukları

- **Görüntüleme zamanı dize kataloğu** (`kitapliklar/arayuz-egui/src/i18n/`, #36 ile geldi): dil başına bir
  TSV (bağlam, İngilizce kaynak, çeviri, `@plural` satırları), Otomatik (sistem dili) ile
  Düzenle → Arayüz Dili, canlı geçiş, kaydedilmiş seçim ve kapsam, yer tutucular ve çoğullar
  için testler ([yerellestirme.md](yerellestirme.md)). Ukraynaca ve Almanca katalogların her biri
  **989 satır** içerir;
  menüleri, komut etiketlerini, araç gruplarını ve araç ipuçlarını, Başlangıç sayfasını, sekmeleri,
  durum denetimlerini, nesne kenetlemelerini, çizim istemlerini, özellik etiketlerini ve iletişim
  kutularını kapsar. Motor tanılamaları, CLI/MCP yanıtları ve komut geçmişi İngilizce kalır ve
  kataloglanmamış metin İngilizceye geri döner. Ölçek için: `kitapliklar/motor` ve `kitapliklar/arayuz-egui`
  içinde yaklaşık 1,160 benzersiz büyük harfle başlayan dize değişmezi vardır (grep, 2026-10-10).
- **Çizim metni:** `skrifa` aracılığıyla TrueType/OpenType/`.ttc`, karakter başına yazı tipi geri
  dönüşü ve FONTALT ile, böylece CJK karakterleri oluşturulur (#173). DXF metni `\U+` kaçışlarıyla
  yazılır, böylece ASCII dışı karakterler R2000 dosyalarında hayatta kalır (#169). Sağdan sola metin
  içeren satırlar Unicode Çift Yönlü Algoritmayı ve `harfrust` ile OpenType şekillendirmeyi alır
  (Arapça, Farsça ve Urduca birleştirme, lam-elif, ZWNJ, işaretler; İbranice), TEXT ve MTEXT içinde
  sarma dahil (#287). Soldan sağa satırlar hâlâ kod noktası başına bir glif yerleştirilir: Devanagari
  ve diğer Hint betikleri için **şekillendirme yok** ve **dikey metin yok**. SHX yazı tipleri
  okunmaz; bizim çizgi yazı tipimiz yerine geçer.
- **Metin girişi:** egui/winit IME. Birkaç upstream IME düzeltmesi PR olarak kuyrukta (#110–#119:
  macOS Korece, emoji seçici, Windows imleç yerleşimi, X11 ve Wayland), bu yüzden CJK girişi henüz
  her platformda güvenilir değil.

## Diller

Sıra ve sütunlar craftrules'ün progress-docs standardından. Her UI yüzdesi kataloglardan
**ölçülür** (katalog dosyası olmayan her dil için 0 satır). Saatler **tahminidir**:
bir Opus 5.5 ajanının ~990 katalog satırını çevirmesi ve yerleşimi düzeltmesi için dil başına
yaklaşık 3–5 saat, artı bir insan tarafından ana dil incelemesi; kataloğu kalan yalnızca İngilizce
dizelere (motor tanılamaları, kataloglanmamış metin) genişletmek yaklaşık 4–6 saat, ilk satırda bir
kez sayılır.

| Dil | Kod | Çevrilen UI dizeleri | İletişim kutuları / araç ipuçları / yardım | Betik desteği | Ana dil incelemesi | Durum | `full` için tahmin |
|---|---|---|---|---|---|---|---|
| İngilizce | en | kaynak (100%) | 100% (yardım bir komut referansıdır, AutoCAD boyutunda yardım değil) | Latin: evet | n/a | full | — |
| Basitleştirilmiş Çince | zh-Hans | 0 (0%) | 0% | CJK glifleri geri dönüşle oluşturulur; IME düzeltmeleri beklemede (#110–#119); dikey metin yok | hayır | none | 8–12 saat (kataloğu genişletme dahil, 4–6 saat) |
| İspanyolca | es | 0 (0%) | 0% | Latin: evet | hayır | none | 3–5 saat |
| Hintçe | hi | 0 (0%) | 0% | Devanagari: şekillendirme yok | hayır | none | 3–5 saat + 15–25 saat şekillendirme (Arapça ile paylaşılır) |
| Arapça | ar | 0 (0%) | 0% | Çizim metni şekillendirilir ve sağdan sola yerleştirilir (#287); UI aynalanmaz | hayır | none | 3–5 saat + 15–25 saat RTL UI |
| Fransızca | fr | ≈ 900 katalog satırı (eksikler bildirilir, İngilizce geri dönüş) | kısmi | Latin: evet | @pechnikov2026-star tarafından katkıda bulunuldu (#624); henüz ana dil incelemesi yok | partial | 2–4 saat |
| Rusça | ru | ≈ 900 katalog satırı (eksikler bildirilir, İngilizce geri dönüş) | kısmi | Kiril: evet (paketlenmiş egui yazı tipleri) | @pechnikov2026-star tarafından katkıda bulunuldu (#624); henüz ana dil incelemesi yok | partial | 2–4 saat |
| Portekizce | pt | 0 (0%) | 0% | Latin: evet | hayır | none | 3–5 saat |
| Endonezce | id | 0 (0%) | 0% | Latin: evet | hayır | none | 3–5 saat |
| Japonca | ja | 0 (0%) | 0% | CJK glifleri oluşturulur; IME beklemede; dikey metin yok | hayır | none | 4–6 saat |
| Almanca | de | 989 katalog satırı (kataloğun ≈ 100%'ü; tüm UI dizelerinin ≈ 80%'i, tahmini: motor tanılamaları ve kataloglanmamış metin İngilizce kalır) | iletişim kutuları ve araç ipuçları evet; komut referansı Almanca aranabilir | Latin: evet (paketlenmiş egui yazı tipleri ä ö ü Ä Ö Ü ß kapsar; uzun dizeler yerleşim kontrolleri gerektirecek) | ana dil incelemesi yok | partial | 1–2 saat |
| Yunanca | el | 1,330 katalog satırı (toplanan tüm mevcut kaynak anahtarları) | Menüler, iletişim kutuları, araç ipuçları, çizim istemleri, komut etiketleri ve bağlamsal başlıklar; kataloglanmamış UI için İngilizce geri dönüş | Yunan betiği; paketlenmiş egui glif kontrolü eklendi | @Camthalion77 tarafından orijinal yapay zekâ destekli çeviri, bağımsız ana dil incelemesi beklemede | mevcut kataloğu tamam; kataloglanmamış UI İngilizce kalır | ana dil düzeltmesi ve platform yerleşim incelemesi |
| Korece | ko | 1,235 satır (1,229 sıradan, 4 menü, 2 sayım); mevcut kataloğun tam birleşimi ve işaretli UI kapsamı | iletişim kutuları, araç ipuçları, Options ve çevrilmiş komut referansı etiketleri | UI, kataloğu kapsayan kurulu veya isteğe bağlı craft-fonts Hang/Kore yüzü gerektirir; olmadan İngilizce geri dönüş; IME düzeltmeleri hâlâ beklemede | orijinal yapay zekâ destekli çeviri; bağımsız ana dil düzeltmesi beklemede | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | ana dil incelemesi + platform yerleşimi/IME doğrulaması |
| Vietnamca | vi | 0 (0%) | 0% | Yığılmış aksanlı Latin: kod noktası başına oluşturulur, test edilmedi | hayır | none | 3–5 saat |
| Ukraynaca | uk | 989 katalog satırı (kataloğun ≈ 100%'ü; tüm UI dizelerinin ≈ 80%'i, tahmini: motor tanılamaları ve kataloglanmamış metin İngilizce kalır) | iletişim kutuları ve araç ipuçları evet; komut referansı Ukraynaca aranabilir | Kiril: evet (paketlenmiş egui yazı tipleri) | bir katkıda bulunan tarafından çevrildi; ikinci inceleme yok | partial | 2–4 saat |
| Çekçe | cs | 1,202 katalog satırı; referans katalog ve işaretli UI değişmezleri kapsandı | iletişim kutuları ve araç ipuçları evet; bağlamsal menü başlıkları; komut referansı Çekçe aranabilir | Latin: evet (paketlenmiş egui orantılı ve tek aralıklı yazı tipleri; Çek harfleri ve tırnakları test edildi) | @Losminek tarafından yapay zekâ destekli; bağımsız inceleme geçişi; henüz ana dili konuşan biri tarafından düzeltilmedi | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | ana dil incelemesi + paylaşılan yalnızca İngilizce UI işi |
| İsveççe | sv | 1,330 katalog satırı; referans katalog, işaretli UI değişmezleri ve menü başlıkları kapsandı (eksik bildirilmedi) | iletişim kutuları ve araç ipuçları evet; bağlamsal menü başlıkları; komut referansı İsveççe aranabilir | Latin: evet (paketlenmiş egui orantılı ve tek aralıklı yazı tipleri; å ä ö é ve İsveç tırnakları test edildi) | @holmbom tarafından yapay zekâ destekli (Claude Code); terminoloji ve çalışan uygulama ana dili konuşan biri tarafından incelendi | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | satır satır düzeltme + paylaşılan yalnızca İngilizce UI işi |
| İtalyanca | it | 1,227 katalog satırı; referans katalog ve işaretli UI değişmezleri kapsandı | iletişim kutuları ve araç ipuçları evet; bağlamsal menü başlıkları (Visualizza / Vista, Edita / Modifica); komut referansı İtalyanca aranabilir | Latin: evet (paketlenmiş egui yazı tipleri; aksanlı harfler test edildi) | Emanuele Bottelli (Claude) tarafından yapay zekâ destekli, İngilizce dizelerden İtalyan masaüstü CAD sözlüğüyle yazıldı; henüz ana dil düzeltme geçişi yok | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | ana dil incelemesi + paylaşılan yalnızca İngilizce UI işi |
| Macarca | hu | 1,326 katalog satırı; referans katalog, işaretli UI değişmezleri ve menü başlıkları kapsandı (eksik bildirilmedi) | iletişim kutuları ve araç ipuçları evet; bağlamsal menü başlıkları; komut referansı Macarca aranabilir | Latin: evet (paketlenmiş egui orantılı ve tek aralıklı yazı tipleri; ő ű Ő Ű ve diğer aksanlı harfler test edildi) | @Skaywalker tarafından İngilizce dizelerden çevrildi; bağımsız ana dil düzeltmesi beklemede | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | ana dil düzeltmesi + paylaşılan yalnızca İngilizce UI işi |
| Türkçe | tr | 1,330 katalog satırı; referans katalog, işaretli UI değişmezleri ve menü başlıkları kapsandı (eksik bildirilmedi) | iletişim kutuları ve araç ipuçları evet; bağlamsal menü başlıkları (Ölçülendirme / Ölçü); komut referansı Türkçe aranabilir | Latin: evet (paketlenmiş egui orantılı ve tek aralıklı yazı tipleri; Ç Ğ İ Ö Ş Ü ç ğ ı ö ş ü test edildi) | @ogzhnklvz tarafından yapay zekâ destekli, İngilizce dizelerden Türk masaüstü CAD sözlüğüyle yazıldı; çalışan uygulama ana dili konuşan biri tarafından incelendi | mevcut kataloğu tamam; yalnızca İngilizce arayüz eksikleri sürüyor | satır satır düzeltme + paylaşılan yalnızca İngilizce UI işi |

Gönderilen diğer diller: Ukraynaca, Brezilya Portekizcesi, Almanca, Çekçe, Rusça, Fransızca,
İtalyanca, Korece, İsveççe, Macarca ve Türkçe.
Çekçe, İsveççe, Macarca ve Brezilya Portekizcesi referans kataloğu ve işaretli her UI değişmezini
kapsar; motor tanılamaları ve kataloglanmamış metin
İngilizce kalır. AutoCAD for Mac 7 İngilizce dışı dil gönderir (zh-Hans, es, fr, ja, de, ko, it);
bunlardan SPCAD artık Almanca ve Korece ve ek olarak AutoCAD for Mac'in göndermediği Ukraynacayı
gönderir.

**On ikisinin tümünü `full` durumuna getirmek için toplam:** yaklaşık 55–95 Opus 5.5 saat (katalog
genişletme 4–6 saat, on çeviri 30–50 saat, karmaşık betik şekillendirme ve RTL 30–45 saat), artı her
dil için ana dili konuşan incelemesi, ki bu insan gerektirir. Katalog var olduğunda çeviriler
tamamen paralelleştirilebilir.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-15 | küçük | Yunanca (`el`) kataloğu: 1,330 satır, bağlam/çoğul/yerel ayar/yazı tipi testleri; inceleme beklemede |
| 2026-10-14 | küçük | İsveççe tamamlandı: o zamandan beri eklenen dizeler için 95 satır (Tarama ve Degrade iletişim kutusu, Blok Düzenleyici, REFEDIT, çoklu kılavuz düzenleme, Options seçim ayarları, MTEXT düzenleyici, son dosyalar); 1,330 satır, Ukraynaca referans kataloğunun her anahtarı |
| 2026-10-18 | küçük | Türkçe eklendi: 1,330 satır (mevcut kataloğun birleşimi), tek sayım biçimi, dört menü başlığı, istem anahtar sözcükleri İngilizce tutuldu; yerel ayar/tercihler/menü/canlı geçiş/iletişim kutusu/yazı tipi testleri; işaretli UI değişmezlerinde eksik yok; yapay zekâ destekli, çalışan uygulama ana dili konuşan biri tarafından incelendi |
| 2026-10-13 | küçük | Macarca eklendi: 1,326 satır, tek sayım biçimi, menü başlıkları, yerel ayar/tercihler/menü/yazı tipi/oluşturma testleri; işaretli UI değişmezlerinde eksik yok; ana dil düzeltmesi beklemede |
| 2026-10-13 | küçük | İsveççe eklendi: 1,235 satır, one/other çoğulları, menü başlıkları, yerel ayar/tercihler/menü/yazı tipi/oluşturma testleri; işaretli UI değişmezlerinde eksik yok; terminoloji ve çalışan uygulama ana dili konuşan biri tarafından incelendi |
| 2026-10-13 | küçük | Korece eklendi: mevcut 1,235 satırlık kataloğun tam birleşimi, tek sayım biçimi, yerel ayar/kalıcılık/canlı geçiş testleri ve doğrulanmış Hangul UI yazı tipleri; ana dil incelemesi ve platform doğrulaması beklemede |
| 2026-10-12 | küçük | İtalyanca eklendi: 1,227 satır, one/other çoğulları, iki `menu` başlığı, yerel ayar/tercih/çoğul/yazı tipi testleri; yapay zekâ destekli, ana dil düzeltmesi beklemede |
| 2026-10-12 | küçük | Brezilya Portekizcesi kataloğu @RicSchonfelder tarafından tamamlandı: 238 satır eklendi (toplam 1,231), artık referans kataloğu ve işaretli her UI değişmezini kapsıyor, eksikler 0; kataloğun aksanlı harfleri için paketlenmiş yazı tipi glif testi |
| 2026-10-11 | küçük | Çekçe eklendi: 1,202 satır, one/few/other çoğulları, yerel ayar/tercihler/menü/yazı tipi/oluşturma testleri; bağımsız inceleme geçişi; ana dil düzeltmesi beklemede |
| 2026-10-11 | küçük | Almanca katalog geldi: `de` tercihi ve kataloğu, dil başına çoğul kuralları; İngilizce dışı diller 1 → 2; toplam saat 60–100 → 55–95 |
| 2026-10-10 | küçük | Sağdan sola çizim metni: TEXT ve MTEXT'te çift yönlü ve Arapça betik şekillendirme (#287) |
| 2026-10-10 | küçük | Ukraynaca katalog main'e geldi (#36): katalog altyapısı var, Ukraynaca kısmi; toplam saat 70–110 → 60–100 |
| 2026-10-10 | büyük | AutoCAD 2027 for Mac'in paket yerelleştirmelerine karşı ilk yerelleştirme ölçümü |
