# Geometri paritesi

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-14 · **Değişiklik:** önemsiz (tek bir yay veya eliptik yay üzerinde JOIN Close) · **Hedef:** Autodesk AutoCAD 2027

CAD kullanıcıları sayılara güvenir: kırpılmış bir uç nokta, bir öteleme, bir kesişim veya bir alan
ya görüntülenen son basamağa kadar doğrudur ya da araç üretim çizimleri için kullanılamaz. Bu belge,
SPCAD'nin geometri çekirdeğini (`kitapliklar/geometri`, `kitapliklar/motor/src/komut` içindeki eğri kodu,
`kitapliklar/kisitlar`) AutoCAD kullanıcılarının güvendiği şeylere karşı eğri eğri denetler. Yakalamalar
ve izleme [arayuz-paritesi.md](arayuz-paritesi.md) içinde; ölçülendirme ve tarama davranışı [eksikler.md](eksikler.md)
içinde.

**Özet (tahmini): geometri ≈ 40% (aşağıda ağırlıklı: 41%), bunun kısıt çözücü ≈ 50%.** Çizgilerden,
yaylardan ve çemberlerden inşa edilen her şey tamdır. Elipsler ve spline'lar örneklenerek işlenir;
bu nedenle bunlar üzerindeki kesişimler, ötelemeler, kırpmalar ve yakalamalar yaklaşıktır; çoklu
çizgi ötelemeleri yay kabarıklıklarını yeniden hesaplamaz veya döngüleri temizlemez; toleranslar
mutlaktır, bu yüzden çok büyük veya çok küçük çizimler orta büyüklükteki çizimlerden farklı davranır.
3B geometri yoktur. Kalan: 2B hassasiyet ve çözücü için **≈ 90–140 Opus 5.5 saat**, ayrıca 3B
çekirdek (M11, bkz. [hedef-uygulama-paritesi.md](hedef-uygulama-paritesi.md)).

## Bu nasıl ölçüldü

2026-10-10 tarihinde kaynaktan okundu (main, bd2412f), tablolarda dosya:satır kanıtı var. AutoCAD'e
karşı henüz sayısal bir karşılaştırma yapılmadı: plan, tam yanıtları analitik olarak bilinen bir dizi
sentetik çizimin her iki uygulamada kara kutu olarak (LIST, ID, AREA sonuçları) ve SPCAD'de başsız
olarak ölçülmesidir. O zamana kadar yüzdeler yargılardır.

Ağırlıklar (AutoCAD kullanıcılarının 2B çizimde her alana ne kadar yaslandığı): sayısal sağlamlık
15%, eğri temsili 10%, kesişimler 15%, ötelemeler 15%, kırpma/uzatma/yuvarlatma/pah/kırma/birleştirme
15%, ölçüm (en yakın nokta, uzunluk, alan, MASSPROP) 10%, tarama sınırları 10%, kısıtlar 10%.

## Sayılar ve toleranslar (≈ 30%)

| Öğe | AutoCAD | SPCAD | Kanıt |
|---|---|---|---|
| Temsil | IEEE double, baştan sona 3B | f64, **2B çekirdek** (`Vec2`); Z yalnızca varlık noktalarında ve 3B çoklu çizgilerde tutulur | `geom/src/vec.rs` |
| Tolerans modeli | Geometriye göreli (ACIS/AcGe tolerans bağlamı), ölçüm koordinatlarında kararlı | tek mutlak `EPS = 1e-9`, ayrıca kesişimlerde geçici mutlak 1e-12/1e-14 ve eğrileri örneklerken 1e-3/1e-4 dünya birimi; birkaç göreli tolerans (çizgi–çizgi paydası, tarama, çözücü) | `geom/src/lib.rs:26`, `intersect.rs:8,19,34,50`, `cmd/modify.rs:469-474`, `snap.rs:98-110` |
| Sağlam yüklemler | Önemli olduğu yerde kesin yön kararları | yok | — |
| Büyük koordinatlar (UTM, 1e6–1e7) | Çalışır | test edilmedi; mutlak toleranslar teğetliği ve çakışıklığı yanlış yargılar ve mutlak örnekleme adımları sınırlara takılan çok büyük örnek sayıları oluşturur | — |
| Nokta birleştirme (JOIN, PEDIT Join fuzz, bölge) | Komut başına fuzz mesafesi | JOIN 1e-9 mutlak; PEDIT'te fuzz var; bölge ızgara anahtarı `tol` değerinde yuvarlanır | `modify.rs:419`, `region.rs:42` |
| Görüntüleme hassasiyeti (LUPREC, AUPREC, birimler) | Ondalık, Mühendislik, Mimari, Kesirli, Bilimsel; 5 açı biçimi | hepsinin birim biçimlendirmesi ve ayrıştırması | `engine/src/units.rs` |

## Eğriler (≈ 60%)

| Eğri | AutoCAD | SPCAD |
|---|---|---|
| Çizgi, ışın, xline | evet | evet, tam |
| Çember, yay, kabarık çoklu çizgi | evet | evet, tam |
| Elips, eliptik yay | evet | evet (merkez, büyük eksen, oran, parametreler) |
| Spline: CV biçimi, rasyonel, herhangi bir derece | evet | evet: rasyonel B-spline, temel fonksiyon değerlendirmesi |
| Spline: uyum noktaları | uç teğetleriyle 3. derece, düğüm parametrelendirmesi (kiriş, karekök, tekdüze), uyum toleransı | kübik, kiriş uzunluğu, ortalanmış düğümler, yoğun çözüm; **uç teğetleri yok, düğüm parametrelendirmesi seçimi yok, uyum toleransı yok** (`geom/src/spline.rs:31-88`) |
| Spline: kapalı/periyodik | periyodik | "closed" ilk noktayı ekler ve yeniden uyumlar; **periyodik değil** (`modify2.rs:647`) |
| Spline düzenleme: düğüm ekleme, bölme, arındırma, yeniden oluşturma, çoklu çizgiye dönüştürme | SPLINEDIT, tutamaçlar | düğüm ekleme ve bölme (`curves.rs:539-622`); arındırma/yeniden oluşturma kısmi |
| Helezon | evet | **hayır** |
| 3B çoklu çizgi | evet | saklanır ve çizilir; 3B işlem yok |
| Bölgeler, 3B katılar, yüzeyler, ağlar (ACIS/B-rep, NURBS yüzeyleri, alt bölme) | evet | **hayır** (M11) |

## Kesişimler (≈ 40%)

| Çift | AutoCAD | SPCAD |
|---|---|---|
| çizgi–çizgi, çizgi–çember/yay, çember–çember | tam | kapalı form, tam; teğetlik sabit bir `|disc| ≤ 1e-12·a` ile kararlaştırılır, bu yüzden teğete yakın durumlar ölçeğe bağlıdır |
| çizgi/yay–elips | tam (kuartik) | **örneklenmiş**: TRIM'de 1e-4 dünya birimi, yakalamalarda 1e-3; TRIM 720 örnek + 60 adımlı ikiye bölme ile arındırır, çizgi/yay kesicilerine karşı neredeyse tam (`modify2.rs:1946-1995`) |
| çizgi/yay–spline | toleransa kadar tam (alt bölme + Newton) | yukarıdaki gibi örneklenmiş |
| elips–elips, elips–spline, spline–spline | toleransa kadar tam | yalnızca **poligon yaklaşımları** |
| Kendi kendini kesişimler (eğriler, öteleme döngüleri) | evet | **yordam yok** |
| Görünür kesişim (projeksiyonlu) | evet | düz kesişim olarak ele alınır |

## Ötelemeler (≈ 30%)

| Eğri | AutoCAD OFFSET | SPCAD (`cmd/modify.rs:276-424`) |
|---|---|---|
| Çizgi, ışın, xline | tam | tam |
| Çember, yay | tam | tam (yarıçap değişimi) |
| Elips | tolerans içinde spline yaklaşımı | 1e-3 × büyük eksende örneklemeden **bir çoklu çizgi** |
| Spline | tolerans içinde spline yaklaşımı | 1e-3'te örnekle, köşeleri normaller boyunca ötele, yaklaşık 40 nokta tut, yeniden uyumla: **kayıplı** |
| Yaylı çoklu çizgi | tam, kendi kendini kesişim kırpması, OFFSETGAPTYPE (uzat, yuvarlat, pahla) | parçalar ötelenir ve komşuların kesişimlerinde birleştirilir; uç noktalar hareket ettikten sonra **yay kabarıklıkları yeniden hesaplanmaz** (L413'teki yorum), **döngü temizliği yok**, sıfır uzunluğa düşmüş parçalar kaldırılmaz; OFFSETGAPTYPE yalnızca yankılanır |
| Seçenekler: Through, Erase, Layer, Multiple | evet | bkz. [eksikler.md](eksikler.md) |

## Kırpma, uzatma, yuvarlatma, pah, kırma, birleştirme, uzunluk değiştirme (≈ 70%)

| Komut | AutoCAD destekler | SPCAD destekler |
|---|---|---|
| TRIM | her eğri tipi, hızlı ve standart kipler | çizgi, yay, çember, çoklu çizgi; elips ve spline örneklenmiş kesme yoluyla (`modify.rs:485-656`, `modify2.rs` `trim_other`); kapalı bir spline bir açık spline bırakır. Kapalı bir çoklu çizgide ilk kesimden önce veya son kesimden sonra yapılan bir seçim, kesimler arasındaki koşuyu tutar ve fazlasını değil (#726). |
| EXTEND | her açık eğri | çizgi, yay, çoklu çizgi, eliptik yay, açık spline (uç teğeti boyunca ilerler, `modify2/curve_edits.rs`) |
| FILLET | çizgiler, yaylar, çemberler, çoklu çizgiler, elipsler, spline'lar, xline'lar, ışınlar; Trim/No trim | çizgiler, yaylar, çemberler (TTR çemberi), çoklu çizgi kipi, Trim/No trim, Multiple; eliptik yaylar ve spline'lar bunlardan herhangi biriyle (örneklenmiş ötelemelerden teğet çember, Newton yöntemiyle arındırılır, `geom/src/fillet.rs`; yaylar elips çevresinde kırpılır veya uzatılır, spline'lar temas noktasında bölünür); bir çoklu çizginin iki düz parçası (komşu, veya aralarında köşenin yerini aldığı bir parça), bir çoklu çizginin düz uç parçası bir çizgiyle (çizgi ona katılır); çoklu çizgi yay parçaları, xline'lar veya ışınlar değil |
| CHAMFER | çizgiler, çoklu çizgiler, xline'lar, ışınlar | çizgiler, Distance/Angle, tüm çoklu çizgiler (Polyline seçeneği), FILLET'teki gibi çoklu çizgi parçaları (`modify2/fillet_pline.rs`); yayları, elipsleri ve spline'ları bir mesajla reddeder, AutoCAD gibi; xline'lar veya ışınlar değil |
| BREAK | her eğri | çizgi, yay, çember, çoklu çizgi |
| JOIN | çizgiler, yaylar, çoklu çizgiler, elipsler, spline'lar, helezonlar | çizgi, yay, çoklu çizgi bir çoklu çizgiye; tek bir elips üzerindeki eliptik yaylar daha büyük bir yaya veya tam elipse (boşluklar kapanır); Close tek bir yayı çemberine ve tek bir eliptik yayı tam elipsine dönüştürür; spline'lar çizgiler, yaylar, çoklu çizgiler ve eliptik yaylarla tek bir tam NURBS'a (derece yükseltme, rasyonel yaylar; `geom/src/nurbs.rs`); helezonlar yok |
| BREAK | her eğri | çizgi, yay, çember, çoklu çizgi; kapalı bir çoklu çizgi, köşeleri boyunca birinci noktadan ikinci noktaya kadar olan parçayı kaybeder ve açık bırakılır (#1173) |
| JOIN | çizgiler, yaylar, çoklu çizgiler, elipsler, spline'lar, helezonlar | çizgi, yay, çoklu çizgi bir çoklu çizgiye; tek bir elips üzerindeki eliptik yaylar daha büyük bir yaya veya tam elipse (boşluklar kapanır); spline'lar çizgiler, yaylar, çoklu çizgiler ve eliptik yaylarla tek bir tam NURBS'a (derece yükseltme, rasyonel yaylar; `geom/src/nurbs.rs`); helezonlar yok |
| LENGTHEN | çizgiler, yaylar, çoklu çizgiler, eliptik yaylar, spline'lar | çizgi, yay, çoklu çizgi; eliptik yay (elips boyunca yay uzunluğu); açık spline (eğri boyunca kesilir veya uç teğeti boyunca ilerler) |

Çizgi ve yayın desteklendiği yerlerde sonuçlar tamdır.

## Ölçüm (≈ 40%)

| Nicelik | AutoCAD | SPCAD |
|---|---|---|
| En yakın nokta: çizgi, yay, çember, çoklu çizgi | tam | tam |
| En yakın nokta: elips, spline | toleransa kadar tam | **örneklenmiş** (`curve.rs:264`, `curves.rs:637`) |
| Teğet noktaları, ortak teğetler | her eğri | yalnızca çemberler ve yaylar (`curve.rs:170-183`) |
| AREA: çember, elips | tam | tam alan (elips çevresi Ramanujan ile, tam değil); açık bir eliptik yayın alanı yoktur, bir yay gibi |
| AREA: yaylı çoklu çizgi | tam (kabarık parçalar) | **1e-4'te örneklenmiş** (`curve.rs:491`) |
| MASSPROP (bölgeler: alan, çevre, ağırlık merkezi, momentler) | bölgelerde ve katılarda tam | örneklenmiş bir çember, çoklu çizgi veya ilk tarama döngüsü üzerinde bağcık yöntemi (`utility.rs:310-352`); bölge yok |
| LIST/DIST/ID hassasiyeti | tam double | tam double |

## Tarama sınırları (≈ 45%)

`geom/src/region.rs`: eğriler poligonlaştırılır, kesişimler bir ızgarada bölünür, köşeler `tol`
değerinde birleştirilir, bir ışın kenarı seçer ve yüz en keskin sola dönüşle izlenir; adalar
bağlantılılıkla (ayrı çizgilerden oluşan adalar dahil, #163). Sınırlar: 60,000 parça, 4,096 ada.
Eksik: **boşluk toleransı (HPGAPTOL)**, sınırlarda gerçek yaylar ve eğriler (sınırlar örneklenmiş
olarak çıkar, bu yüzden taranmış bir çemberin sınırı bir poligondur), varsayılanın ötesinde ada
algılama kipleri, sınır kümeleri ve her durum için korunan ilişkisel sınırlar. Desen çizgileri,
örneklenmiş döngülere karşı kırpılan tarama çizgisi aileleridir (`render/src/hatch.rs`).

## Kısıt çözücü (≈ 50%)

| Öğe | AutoCAD | SPCAD (`kitapliklar/kisitlar`) |
|---|---|---|
| Geometrik kısıtlar (12) | coincident, collinear, concentric, fix, parallel, perpendicular, horizontal, vertical, tangent, smooth (G2), symmetric, equal | 12'sinin tümü; Smooth, bir spline ile bir çizgi, yay, eliptik yay, çoklu çizgi parçası veya spline arasında G2'dir (ortak nokta, teğet ve eğrilik, `curves.rs`); iki spline olmayan eğri arasında teğetlik olarak kalır |
| Boyutsal kısıtlar | doğrusal, yatay, düşey, hizalı, açısal, yarıçap, çap; dinamik ve notlandırmalı; referans | mesafe (hizalı/yatay/düşey), açısal, yarıçap, çap; parametreler ve ifadeler (`expr.rs`) |
| Geometri | çizgiler, yaylar, çemberler, çoklu çizgiler, elipsler, spline'lar, bloklar | çizgi, çember, yay, çoklu çizgi, nokta; elips ve eliptik yay (merkez, eksen uçları ve yay uçları nokta olarak; büyük eksen üzerinde teğet, çakışık, eşmerkezli, yatay/düşey); spline (uyum veya kontrol noktaları nokta olarak; uçlarında teğet ve düzgün; açık kenetlenmiş veya uyum spline'ları); blok referansı (ekleme noktası ve bloğun kendi noktalarının konum ve dönüşle katı biçimde birlikte hareket etmesi) (`model.rs`) |
| Algoritma | tescilli | sönümlü Gauss–Newton / Levenberg–Marquardt, sayısal Jacobian, bağlı bileşen başına; MAX_ITER 200, MAX_ROWS 800; elipsle eğri–eğri teğetliği yardımcı temas parametreleri aracılığıyla |
| Aşırı kısıtlama | bir mesajla reddedilir | Jacobian rankına göre gereksizlik (sütun pivotlamalı Householder QR, satırlar ve sütunlar dengelenir), Fix dahil; mesaj yeni kısıtın hangi kısıtları çoğalttığını adlandırır; çakışmalar kaldırma yoklamasıyla (en fazla 48 yoklama) |
| Serbestlik dereceleri, tam kısıtlı durum | gösterilir (kısıt çubukları, durum) | `constraints.dof` ve `constraints.inspect`: bileşen başına serbest parametreler eksi Jacobian rankı, sıfır uzayından nesne başına DOF, eksik/tam/aşırı kısıtlı durum (`dof.rs`); her kısıt çubuğunda bir durum şeridi ve tuvalde çizimin durumu |
| Çizim sırasında çıkarım (CONSTRAINTINFER) | evet | CONSTRAINTINFER (değişken ve komut): bir çizim veya düzenleme komutundan sonra, mevcut geometriye tam olarak yerleştirdiği şeyler (yakalamalar, orto, kutupsal) Coincident (uç, orta, merkez, nesne üzerinde), Tangent, Perpendicular, Parallel ve Horizontal/Vertical kısıtları alır, AutoConstrain tiplerine göre süzülür (`auto.rs` `infer_placed`) |

## Testler

`#[test]` sayıları 2026-10-10'da: geom 28, constraints 22, render 50, engine 174. Geometri sayıları
için özellik tabanlı ve düşmanca girdili bir takım 2026-10-12'de eklendi
(`kitapliklar/geometri/src/ozellik_testleri.rs`): deterministik bir xorshift üreteci (yeni bağımlılık yok)
özellik başına birkaç bin durum sürer — kesişimler her iki eğri üzerinde yer alır ve simetriktir,
çember çiftleri kapalı formla eşleşir, en yakın noktalar sabit noktalardır (parça içinde dik, uçlar
dışında), üç noktalı yaylar `{start, end} = {p1, p3}` korur ve p2'yi içerir (yalnızca saat yönünün
tersine depolama, p2 saat yönü tarafındayken çifti değiştirir, DXF'in yaptığı gibi) ve düşmanca
girdiler (NaN, sonsuzlar, alt normal sayılar, ham bit desenleri, 1e300) asla paniklemez ve yalnızca
sonlu sonuç döndürür veya hiç döndürmez. Bilinen yanıtlar 1e6–1e7 koordinatlarda 1e-9 göreli
doğrulukla geçerlidir.

## Önce ne yapmalı (sıralı)

1. Tam elips ve spline kesişimi ve en yakın nokta (göreli toleransla alt bölme + Newton); bunları
   yakalamalarda, TRIM, EXTEND, BREAK'te kullan. 15–25 saat.
2. Kabarıklık yeniden hesaplama, kendi kendini kesişim temizliği ve OFFSETGAPTYPE ile çoklu çizgi
   ötelemesi; tolerans içinde uyumlanmış spline'lar olarak spline/elips ötelemesi. 15–25 saat.
3. Çizimin sınırlarına göreli bir tolerans bağlamı ve orijinde ve 1e6–1e7 koordinatlarda bilinen
   yanıtlı bir test takımı. 8–12 saat.
4. Kabarık çoklu çizgiler, bölgeler (2B) ve tarama döngüleri için tam alanlar ve MASSPROP. 6–10 saat.
5. Tarama boşluk toleransı ve eğri sınırlar. 8–12 saat.
6. ~~Çözücü: DOF raporlama, G2 Smooth, elips/spline geometrisi, kısıt çıkarımı.~~ Tamamlandı (#546).
7. Spline uyum seçenekleri (uç teğetleri, düğüm parametrelendirmesi, uyum toleransı, periyodik). 6–10 saat.

Toplam ≈ 90–140 saat; 1–6 maddeleri iki veya üç ajan arasında paralelleştirilebilir.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-14 | önemsiz | JOIN Close: tek bir yay çemberine, tek bir eliptik yay tam elipsine dönüşür (#1175) |
| 2026-10-12 | önemsiz | Kapalı bir çoklu çizgide TRIM, seçim son kesimin ötesindeyken yalnızca tamamlayıcı koşuyu tutar (#726) |
| 2026-10-12 | küçük | Geometri sayıları için özellik tabanlı ve düşmanca girdili testler eklendi (`kitapliklar/geometri/src/ozellik_testleri.rs`): kesişimler, en yakın noktalar, üç noktalı yaylar, sınırlar, vektör özdeşlikleri; düşmanca f64'ler asla paniklemez; 1e6–1e7'de bilinen yanıtlar |
| 2026-10-10 | büyük | İlk geometri hassasiyeti denetimi: toleranslar, eğriler, kesişimler, ötelemeler, değiştirme komutları, ölçüm, tarama, çözücü |
| 2026-10-11 | küçük | Eliptik yaylarda ve spline'larda EXTEND, JOIN ve LENGTHEN; kapalı spline'lar tek parçaya kırpılır; kırpma/uzatma bölümü 45% → 55% (#466) |
| 2026-10-11 | küçük | Eliptik yaylar ve spline'larla FILLET; kırpma/uzatma bölümü 55% → 65% (#466) |
| 2026-10-11 | küçük | Çoklu çizgi parçalarıyla FILLET/CHAMFER (aynı çoklu çizgi, bir çizgiyle uç parçası); bölüm 65% → 70% (#466) |
| 2026-10-11 | küçük | Kısıt çözücü (#546): DOF raporu, G2 Smooth, elips/spline/blok geometrisi, CONSTRAINTINFER |
