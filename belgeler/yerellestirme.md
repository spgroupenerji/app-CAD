# Arayüz dilleri

**Düzenle → Arayüz Dili** seçeneğini seçin ve **Otomatik (sistem dili)**, **English**,
**Українська**, **Português (Brasil)**, **Deutsch**, **Español**, **Čeština** (Çekçe), **Русский**, **Français**, **简体中文**, **Italiano**, **한국어** (Korece), **Svenska** (İsveççe), **العربية** (Arapça), **Magyar** (Macarca), **Türkçe** veya **Ελληνικά** (Yunanca) seçin.
Arayüz, yerel macOS menüleri dahil, yeniden başlatmadan güncellenir.
Seçim, diğer arayüz tercihleriyle birlikte kaydedilir (`spcad.prefs`, temanın yanında):
masaüstünde eframe depolaması, web'de tarayıcı yerel depolaması. Kaydedilmiş seçimi olmayan
kurulumlar Otomatik kullanır.

Otomatik, işletim sisteminin tercih edilen yerel ayarlarında desteklenen ilk dili veya web'de
tarayıcının `navigator.languages` değerini kullanır. BCP 47 ve POSIX etiketleri birincil dillerini seçer:
`uk-UA` → `uk`, `de_AT.UTF-8` → `de`, `es-MX` → `es`, `pt_BR.UTF-8` → `pt-BR`,
`cs-CZ` ve `cs_CZ.UTF-8` → `cs`, `it-IT` ve `it_CH.UTF-8` → `it`, `ko-KR` ve `ko_KR.UTF-8` → `ko`,
`sv-SE`, `sv-FI` ve `sv_SE.UTF-8` → `sv`, `hu-HU` ve `hu_HU.UTF-8` → `hu`, `tr-TR` ve `tr_TR.UTF-8` → `tr`. Desteklenmeyen diller İngilizceye geri döner.
Bir başlatma için masaüstünün yerel ayarını geçersiz kılmak üzere `SPCAD_LOCALE=cs-CZ` ayarlayın.
Açıkça kaydedilmiş bir dil seçimi bu geçersiz kılmanın önüne geçer.

Otomasyon, dili mevcut kontrol kanalı aracılığıyla seçebilir:

```json
{"id":1,"method":"engine.execute","params":{"command":"ui.language","params":{"lang":"cs"}}}
```

`ui.set` yöntemi ayrıca `{"interfaceLanguage":"cs"}` kabul eder. Desteklenen değerler `auto`,
`en`, `uk`, `pt-BR`, `de`, `es`, `cs`, `ru`, `fr`, `zh`, `it`, `ko`, `sv`, `ar`, `hu`, `tr` ve `el`; geçersiz değerler mevcut
seçimi değiştirmeden reddedilir. Paletleri sıfırlamak dil seçimini korur.

## Çeviri kapsamı

Kataloglar komut ve menü etiketlerini, araç gruplarını ve araç ipuçlarını, Başlangıç sayfasını,
dosya ve model alanı sekmelerini, durum denetimlerini, nesne kenetlemelerini, çizim istemlerini,
özellik etiketlerini, iletişim kutularını ve Hakkında penceresinin denetimlerini kapsar. Komut
referansı İngilizce komut kimliğine, İngilizce etikete veya çevrilmiş etikete göre aranabilir.
Ukraynacada zorunlu kapsam vardır;
diğer kataloglardaki eksik girdiler testler tarafından bildirilir ve İngilizceye geri döner.

Çekçe 1,202 katalog satırına sahiptir; referans kataloğu ve Options dahil işaretli her UI
değişmezini kapsar. Dört `menu` girdisi menü başlıklarını nesne/komut
etiketlerinden ayırır: **Zobrazení / Pohled**, **Kreslení / Kreslit**, **Kótování / Kóta** ve **Modifikace / Upravit**.
Çeviri @Losminek'in yapay zekâ destekli çalışmasıdır; Çek masaüstü ve CAD UI
geleneklerine karşı ve bağımsız bir inceleme geçişiyle gözden geçirilmiştir; henüz ana dili konuşan biri tarafından düzeltilmemiştir.

Korece **1,235 satır** içerir: 1,229 sıradan etiket, dört bağlamsal menü başlığı ve
iki tek biçimli sayım şablonu. Ukraynaca referans, mevcut Çekçe katalog, Options, istemler, iletişim kutuları ve araç ipuçları dahil mevcut katalogların birleşimini kapsar.
Çeviriler özgün, yapay zekâ destekli Kore CAD terminolojisidir (örneğin, 도면층,
치수, 모깎기, 모따기, 간격띄우기 ve 객체 스냅). Bağımsız ana dil düzeltmesi ve
platforma özgü yerleşim/IME doğrulaması takip işi olarak kalır.

Brezilya Portekizcesi 1,231 katalog satırına sahiptir ve ayrıca referans kataloğu ve işaretli her UI
değişmezini kapsar: komutlar, menüler, araç grupları ve araç ipuçları, nesne kenetlemeleri, çizim istemleri, iletişim kutuları ve
Preferences; eksikleri artık sıfırdır. Çoğul kuralı `one|other` (0 ve 1 `one`), CLDR'in
`pt` için tanımladığı gibi ve kataloğun aksanlı harfleri (`Á Â Í Ó Ú à á â ã ç é ê í ó ô õ ú`) paketlenmiş
yazı tiplerinde oluşturulur. Orijinal katalog @WladmirJunior tarafından katkıda bulunuldu; kalan satırlar
@RicSchonfelder tarafından yapay zekâ desteğiyle tamamlandı ve henüz ana dili konuşan biri tarafından düzeltilmedi.

Macarca **1,326 satır** içerir: 1,320 sıradan etiket, dört bağlamsal menü başlığı ve iki
tek biçimli sayım şablonu. Options, istemler, iletişim kutuları ve araç ipuçları dahil referans kataloğu ve işaretli her UI değişmezini
kapsar; menü başlığı ile nesne etiketi arasında yalnızca **Méretezés / Méret** (Ölçülendirme) farklıdır. Arayüz kullanıcıya
samimi biçimde hitap eder ve bir Macarca sayıdan sonra tekil geldiği için sayım tek
biçim alır. Çeviri, SPCAD'nin İngilizce dizelerinin anlamından yazıldı; başka bir ürünün yerelleştirmesi
kaynak olarak kullanılmadı. Henüz bağımsız bir ana dili konuşan biri tarafından düzeltilmedi.

Türkçe **1,330 satır** içerir: mevcut katalogların birleşimi, dört bağlamsal menü başlığı
(Dimension menüsü için **Ölçülendirme**, nesne için **Ölçü**) ve iki tek biçimli sayım
şablonu, çünkü Türkçe bir isim sayıdan sonra tekil kalır ("3 commit"). İşaretli hiçbir UI değişmezi
İngilizceye geri dönmez. İstemler Enter için "… girin", Specify için "… belirtin" okunur ve tıklanabilir
anahtar sözcükler (`Top`, `Zero`, `Bottom`, `_tan to`, `nX`/`nXP`) motorun kabul ettiği yazımı korur.
Çeviri @ogzhnklvz'in özgün, yapay zekâ destekli çalışmasıdır; İngilizce dizelerden
Türk masaüstü CAD sözlüğüyle (Katman, Tarama, Yuvarlatma, Pah, Öteleme, Nesne Kenetleme,
Görünüm Penceresi) yazıldı; çalışan uygulama ana dili konuşan biri tarafından incelendi ve satır satır
düzeltme hâlâ yapılacak. Paketlenmiş
egui yazı tipleri `Ç Ğ İ Ö Ş Ü ç ğ ı ö ş ü` kapsar.

Çizim başlıkları, katman/yerleşim/blok/stil adları, parametre adları ve ifadeleri, bir
çizimdeki metin ve komut satırı girdisi kullanıcı verisidir ve özgün yazımlarını korur. Tıklanabilir
komut anahtar sözcükleri İngilizce yazımlarını korur, böylece gösterilen metin motorun kabul ettiği
girdiyle eşleşir. CLI/MCP/kontrol yanıtları, motor tanılamaları ve komut geçmişi İngilizce kalır.
Kataloglanmamış metin de İngilizceye geri döner. Sayı biçimlendirme ve işletim
sisteminin kendi dosya seçici denetimleri mevcut davranışı izler.

Paketlenmiş egui yazı tipleri Ukraynacayı (`Ґґ Єє Іі Її`), Almancayı (`ÄÖÜ äöü ß`), Çekçeyi
(`ÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ áčďéěíňóřšťúůýž`, tipografik tırnaklar `„…“` dahil) ve Macarcayı
(`ÁÉÍÓÖŐÚÜŰ áéíóöőúüű`, `„…”` dahil) kapsar.
Çeviriler hiçbir yazı tipi varlığı eklemez ve web'de sistem yazı tipleri olmadan çalışır.

Korece, **kataloğundaki her Hangul karakterini** ve dil seçici etiketini kapsayan bir yazı tipine ihtiyaç duyar. Her dilde yüklenen CJK yazı tipi (Noto Sans CJK, Microsoft YaHei, PingFang)
genellikle Hangul'u zaten kapsar ve o zaman başka bir şey yüklenmez. Aksi halde, yalnızca Korece
seçildiğinde masaüstü derlemeleri isteğe bağlı `CRAFT_FONTS_DIR` girdisinden bir `Hang`/`Kore` yüzü arar,
sonra kurulu yazı tiplerini: Windows'ta Malgun Gothic/Gulim, macOS'te Apple SD Gothic Neo/AppleGothic
ve Linux/FreeBSD'de Nanum Gothic veya Noto CJK. Koleksiyon yüzleri tek tek kontrol edilir ve
dosya okumaları/yüz taramaları sınırlıdır. Hiçbir sistem yazı tipi veya yeni yazı tipi varlığı yeniden dağıtılmaz.

Web derlemeleri kurulu yazı tiplerini okuyamaz ve (barındırma boyut sınırları içinde kalmak için) Korece
bir yüz içermez, bu yüzden Korece seçildiğinde web arayüzü İngilizce kalır. Kataloğu kapsayan bir yazı tipi
yoksa kaydedilen `ko` tercihi korunur ama arayüz İngilizceye geri döner; dil
komutu bir yazı tipinin gerektiğini bildirir ve seçici, Hangul'un gösterilemediği her yerde okunabilir **Korean** etiketini kullanır. Bu değişiklik platform IME sorunlarını düzeltmez veya motor çıktısını yerelleştirmez.

## Yunanca (`el`) arayüz

Yunanca çeviri, mevcut dil kataloglarından toplanan **1,330** farklı kaynak anahtarının tümünü kapsar:
**1,324** sıradan arayüz etiketi, **4** bağlamsal menü başlığı ve
**2** tam sayı sayım şablonu. Arayüz, Düzenle → Arayüz Dili altında **Ελληνικά** olarak veya sistem dili `el-GR` ya da `el-CY` olduğunda otomatik olarak seçilebilir.
Yunanca tam sayı çoğulları 1'i diğer tüm sayımlardan ayırır.

Terminoloji yerel Yunanca CAD sözlüğünü izler; uzmanlık terimleri, profesyonellerin bilinen bir CAD komutuyla eşleştirmesine yardımcı olduğunda İngilizce
adı parantez içinde korur (örneğin,
**Επίπεδο (Layer)**, **Πολυγραμμή (Polyline)** ve **Διαγράμμιση (Hatch)**).
Open ve Save gibi yaygın eylemler yalnızca Yunanca görünür. Mevcut komut tanımlayıcıları,
çizim içeriği, kullanıcı nesnelerinin adları ve dosya biçimleri değişmez. Çeviri
@Camthalion77 tarafından dil incelemesiyle katkıda bulunulan özgün yapay zekâ destekli çalışmadır; Autodesk yerelleştirme dosyalarından alınmamıştır. Bağımsız ana dil düzeltmesi ve görsel kontroller takip
işi olarak kalır.

## Çeviri ekleme

Uygulama, [VectorCraft #205](https://github.com/storytold/vectorcraft/pull/205),
[PdfCraft #106](https://github.com/storytold/pdfcraft/pull/106) ve
[FilmCraft #238](https://github.com/storytold/filmcraft/pull/238) tarafından kullanılan görüntüleme zamanı katalog desenini izler. Kodu SPCAD'ye özeldir.

- `kitapliklar/arayuz-egui/src/i18n/*.tsv` UTF-8'dir ve sekmeyle ayrılmış üç sütun içerir:
  `context`, `English source` ve `translation`. Sıradan etiketlerin bağlamı boştur.
  İsteğe bağlı `menu` girdileri menü başlıklarını `i18n::menu` / `i18n::tr_menu` aracılığıyla çevirir;
  biri yoksa sıradan çeviri kullanılır.
- `@plural` girdileri dilin tam sayı sayımları için biçimlerini tutar: Ukraynaca `one|few|many`,
  Almanca ve İspanyolca `one|other`, Brezilya Portekizcesi `one|other` (0 ve 1 `one`),
  Çekçe `one|few|other` (1; 2–4; 0, 11, 21 ve 22 dahil diğer her şey).
  Korece ve Macarca tüm tam sayı sayımları için tek biçime (`other`) sahiptir.
  API `u64` alır, bu yüzden kesirli sayımlar kabul edilmez.
- Yeni bir kataloğu `i18n::LANGUAGES` içine kaydedin, tercih varyantını ve kod/adını ekleyin
  ve `Language` satırında tam sayı çoğul kuralını ve biçim sayısını ayarlayın.
- Bir değişmez için `crate::tl!("English label")`, bilinen dinamik bir UI etiketi için `i18n::t(label)`
  ve adlandırılmış yer tutucular için `crate::tf!("Version {version}", version = value)` kullanın.
  Yalnızca görüntüleme sınırında çevirin; rastgele kullanıcı adlarını veya tanımlayıcıları asla aramayın.
- Pencereler İngilizce kaynaklarına dayalı açık kimlikleri korur ve araç grupları kaynak
  adlarını anahtar olarak korur, böylece dil değiştirmek durumlarını korur.
- `ATTRIBUTION.md` içine bir çeviri atıf satırı ekleyin, dil listelerini güncelleyin ve
  yerel ayar, çoğul, yazı tipi glifi ve canlı geçiş testleri ekleyin. Kataloğu
  `i18n/tests.rs` içindeki `REPORTED` listesine ekleyin: `REQUIRED` içinde yalnızca referans çeviri olan Ukraynaca vardır, bu yüzden bir
  komut eklemek tüm dillerde değil, bir dilde satır gerektirir.
- Kataloglar gömülür ve bir kez ayrıştırılır. Eksik veya bozuk satırlar uygulamayı çökertemez;
  testler geçersiz satırları, yinelenen anahtarları, değiştirilmiş yer tutucuları ve uyuşmayan üç noktaları reddeder.
- `cargo test -p spcad-ui-egui i18n` menü/komut/araç/kenetleme kapsamını, işaretli her UI
  değişmezini, yerel ayar çözümlemesini, tercihleri, tam sayı çoğullarını, paketlenmiş glifleri ve dil değiştirmeli gerçek
  egui oluşturmalarını kontrol eder. `cargo xtask assets` varlık atfını kontrol eder;
  `cargo xtask ci` çalışma alanı kalite geçitlerini çalıştırır.

Çeviriler `MIT OR Apache-2.0` altında özgün çalışmadır ve
SPCAD'nin İngilizce dizelerinin anlamından yazılmıştır. Tescilli yerelleştirme kaynaklarını kopyalamayın.
