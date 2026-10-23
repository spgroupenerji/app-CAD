# Hedef uygulama paritesi: SPCAD ile AutoCAD 2027

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-12 · **Değişiklik:** küçük (statik bloklar için Blok Düzenleyici; bloklar kanıtı güncellendi, sayılar yeniden ölçülmedi) · **Hedef:** Autodesk AutoCAD 2027

SPCAD'in AutoCAD'e ne kadar yakın olduğunun ve ne kadar Opus 5.5 işinin kaldığının yetkili
değerlendirmesi. [YOL-HARITASI.md](../YOL-HARITASI.md) bunu özetler, [eksikler.md](eksikler.md) her eksiği listeler ve
derin kontrol listeleri [geometri-paritesi.md](geometri-paritesi.md), [arayuz-paritesi.md](arayuz-paritesi.md),
[dosya-bicimi-paritesi.md](dosya-bicimi-paritesi.md), [donanim-paritesi.md](donanim-paritesi.md) ve
[yerellestirme-paritesi.md](yerellestirme-paritesi.md).

## Öne çıkan sayılar

| Sayı | Değer | Tür |
|---|---|---|
| **Özellik genişliği** (canlı komutu olan AutoCAD for Mac menü öğeleri) | **245 / 491 = 50%** (233 = 47%, yalnızca motor komutlarını gören `cargo xtask parity` ile sayıldı; +12 yalnızca arayüz öğesi) | ölçüldü ([parite-kontrol-listesi.md](parite-kontrol-listesi.md), kaynak taraması) |
| Yalnızca 2B menülerde özellik genişliği (150 3B öğesi hariç) | **≈ 65%** (`cargo xtask parity` kuralına göre 222 / 341) | ölçüldü (kaynak taraması, 2026-10-10) |
| Kullanıma göre ağırlıklı özellik derinliği (özellikler boyutu) | **≈ 41%** | tahmini (aşağıdaki tablo) |
| **Gerçek işe hazır** | **≈ 38%** | tahmini (aşağıdaki boyut tablosu; ağırlıklı 37.7%) |
| Ana akım uygulayıcı (2B teknik ressam, haftalık araçlar) | **≈ 28%** | tahmini ([aşağıda](#ana-akım-uygulayıcı)) |
| Temel kullanıcı (çekirdek araçlar, varsayılan ayarlar) | **≈ 46%** | tahmini ([aşağıda](#temel-kullanıcı)) |
| Alfa'ya kalan (kapı satırları) | **≈ 125–190 Opus 5.5 saati** | tahmini |
| Betaya kalan | **≈ 600–930 Opus 5.5 saati** | tahmini |
| Tam pariteye kalan | **≈ 1,300–2,100 Opus 5.5 saati** | tahmini |

Aşama: **alfa öncesi**. Beş temel iş akışından dördü [alfa kapısı](yol-haritasi.md#alfa-geçidi)
eşiğini geçemiyor (izlemeyle hassas çizim, bloklar, çizdirme, kayıpsız DWG kaydetme) ve gerçek
işe hazırlık ~40% alfa çubuğunun altında. Alfa ≈ 125–190 sa uzakta.

## Hedef kitleye göre hazırlık

| Hedef kitle | Hazır % | ~95%'e Opus 5.5 ajan duvar saati | Baskın iş |
|---|---:|---|---|
| Tam hedef (gerçek işe hazır) | ≈ 38% | ≈ 1,150–1,850 sa (≈ 70%'i 4–6 ajan arasında paralelleşir: ≈ 300–480 sa geçen süre) | 3B çekirdek (300–450 sa), DWG/DXF ve diğer biçimler, AutoLISP ve ekosistem, bloklar/dış referanslar (xref'ler)/dinamik bloklar, yerelleştirme |
| Ana akım uygulayıcı (2B teknik ressam) | ≈ 28% | ≈ 570–880 sa (≈ 70%'i paralelleşir: ≈ 150–240 sa geçen süre) | kayıpsız DWG değişimi ve gerçek dosya korpusu, hassas girdi (izleme, dinamik girdi), 2B geometri hassasiyeti, bloklar/xref'ler, notlandırma derinliği, kararlılık birikimi |
| Temel kullanıcı | ≈ 46% | ≈ 140–225 sa (≈ 50%'si 3 ajan arasında paralelleşir: ≈ 60–110 sa geçen süre) | temel çizim/düzenleme komutlarındaki taslak seçenekler, insanların gönderdiği DWG'leri açma, başlatma/kararlılık düzeltmeleri, çizdirme iletişim kutusu, bağlam menüleri ve dinamik girdi |

Saatler alt kümelerdir (temel ⊂ ana akım ⊂ tam) ve [Kalan emek](#kalan-emek-ve-nasıl-kalibre-edildiği)
bölümündeki kalibrasyonu kullanır: ilk derlemede varlık kalitesinde ajan-saati başına ≈ 25 komut
(≈ 11.5 ajan-saati), seçenek veya hata düzeltmesi başına 0.3–0.7 sa (2026-10-10'da ≈ 60 bu tür
düzeltme), orta ölçekli özellik başına 2–5 sa, alt sistem başına 15–100 sa.

Tam sayı, boyutlar üzerinde toplamsal ağırlıklı bir toplamdır (aşağıdaki tablo); yalnızca ana akım ve
temel kullanıcı sayıları çarpımsal indirimler kullanır.

## Hedef ve ölçüm yöntemi

- **Hedef:** Mac için Autodesk AutoCAD 2027, sürüm 26.0 (`/Applications/Autodesk/AutoCAD 2027`
  altında kurulu) ve ikisinin farklılaştığı yerlerde Autodesk'in belgelerinden AutoCAD 2027 for
  Windows (Ribbon, Sheet Set Manager, Express Tools, .NET).
- **Temiz oda:** kurulu paket yalnızca **adlar listelenerek** incelendi: yerelleştirmeler
  (`Base`, `de`, `es`, `fr`, `it`, `ja`, `ko`, `zh_CN` `.lproj` klasörleri), `Contents/PlugIns`
  modül adları (AcSolids, AcDim, AcTable, AcSection, AcSubD, AcPublish, AcETransmit, AcXDiff /
  DWG Compare, AcCounting, AcMarkupAssist, AcTrace, AcSmartCenter, AcSpeller, Vlide …, yaklaşık 130
  modül) ve `Resources/Support` dosya adları. Hiçbir dosyanın içi okunmadı, hiçbir şey
  kopyalanmadı, bu geçişte AutoCAD başlatılmadı.
- **Menü oracle'ı:** `xtask/veri/menu-catalog.txt` (yalnızca özellik adları, 491 yaprak öğe, çalışan
  AutoCAD 2027 for Mac'ten kaydedildi) komut kaydına karşı.
- **Davranış referansı:** `plan/autocad/01-observed-ui.md` ve yerel referans ekran görüntüleri (kara
  kutu, asla commit edilmez); Autodesk'in kamuya açık komut referansı.
- **SPCAD:** `main` üzerindeki kaynak, bd2412f (2026-10-10/11), dört ajan tarafından paralel
  okundu: komut bazında seçenek denetimleri ("not available yet" yazan istemler), geometri
  çekirdeği, DXF/DWG yolları, etkileşim kodu. Ayrıca 88 GitHub sorunu ve 47 açık PR. Disk neredeyse
  doluydu, bu yüzden hiçbir şey derlenmedi veya çalıştırılmadı: `cargo xtask parity`, xtask'ın
  eşleştirme kuralıyla `CommandSpec` menülerinin bir Python taramasıyla yeniden üretildi ve son
  üretilen raporla tamamen aynı kapsanmayan listeyi verdi (233 / 491). Doğrulamak için
  `cargo xtask parity` yeniden çalıştırın.

## Ana akım uygulayıcı

Tipik AutoCAD profesyoneli bir 2B teknik ressam veya detaylandırıcıdır: her hafta çizer, düzenler,
ölçülendirir, bloklar kullanır, katmanları yönetir, sayfaları çizdirir ve meslektaşlarıyla DWG
alışverişi yapar. Standart gereği hariç tutulanlar: 3B, AutoLISP ve eklentiler, bulut/yapay zekâ
(Autodesk Assistant, Smart Blocks, Traces, Share), Sheet Set Manager yönetimi, özel donanım,
İngilizce dışındaki diller.

| Alan (haftalık kullanım) | Ağırlık | Derinlik | Ağırlıklı |
|---|---:|---:|---:|
| Çizim (2B) | 15% | 60% | 9.0 |
| Düzenleme | 18% | 50% | 9.0 |
| Hassas girdi (yakalamalar, izleme, koordinat girişi, dinamik girdi) | 13% | 40% | 5.2 |
| Katmanlar ve özellikler | 8% | 70% | 5.6 |
| Ölçülendirme | 10% | 45% | 4.5 |
| Metin ve MTEXT | 6% | 35% | 2.1 |
| Kılavuzlar ve tablolar | 3% | 25% | 0.8 |
| Tarama | 5% | 35% | 1.8 |
| Bloklar, nitelikler, xref'ler | 8% | 25% | 2.0 |
| Yerleşimler ve çizdirme | 8% | 35% | 2.8 |
| Sorgu ve yardımcılar (otomatik kaydetme dahil) | 3% | 30% | 0.9 |
| Komut satırı ve gezinme | 3% | 58% | 1.7 |
| **Ortalama derinlik** | 100% | | **≈ 45.3%** |

Derinlikler, yukarıdaki ve [arayuz-paritesi.md](arayuz-paritesi.md) içindeki alan yüzdeleridir. Dosya G/Ç burada
bir alan değildir; dosya değişimi indirimi olarak bir kez sayılır.

| İndirim | Faktör | Kanıt |
|---|---:|---|
| Etkileşim sadakati (hassas girdi alanının ötesinde) | ×0.90 | sıcak tutma noktası seçenekleri yok sayılıyor, bağlam menüleri yok, W/C/F/WP/CP seçim anahtar sözcükleri reddediliyor (#224), saydam 'ZOOM çalışan komutu iptal ediyor (#220), ELLIPSE/BREAK anahtar sözcükleri yok sayılıyor (#219) |
| Gerçek makinelerde kararlılık | ×0.85 | 37 açık sorundan 27'si çekirdek yol hataları; yalnızca 2026-10-10'da yaklaşık 60 doğruluk düzeltmesi birleştirildi; komut ortasında görünmeyen çizgiler (#44); kilitli katmanlar düzenlendi (#87); hiçbir CI, PR'larda Rust testlerini çalıştırmıyor |
| AutoCAD kullanıcılarıyla dosya alışverişi | ×0.80 | kaydetmeler modellenmemiş varlıkları, görüntüleri, katman durumlarını, çoklu kılavuz stillerini, gerçek renkleri ve daha fazlasını düşürüyor (#79, #201, #204, #208, #209, #213, #223); AutoCAD'de hiç kontrol edilmedi; 10 MB'lık bir DWG başarısız oldu (#57) |

**Ana akım uygulayıcı ≈ 45.3% × 0.90 × 0.85 × 0.80 ≈ 28%** (tahmini). SPCAD için gerçek işe
hazırdan (≈ 38%) daha düşüktür, çoğu uygulamanın aksine: tam sayı, SPCAD'in 2B çizimin ötesinde iyi
yaptıklarını (platformlar 75%, performans 60%, donanım 45%) toplamsal olarak hesaba katar, burada ise
çalışan bir teknik ressamın her gün karşılaştığı üç engelleyici çarpılır.

**Kullanıcı kanıtı** (GitHub sorunları, PR'lar ve yorumlar, bakımcılar hariç, 2026-10-10'da sayıldı):
97 sorun açıldı (60 kapalı, 37 açık) ve 19 dış yazardan 127 PR. Övgü: 2 (#65: "I was looking for
Autocad replacement for years"; #18), ayrıca bir çeviri PR'ında teşekkür (#218). "Switched from
AutoCAD" bildirimleri: 0. Açık sorunlar: 27 çekirdek yol hatası (dosya verisi kaybı, çizdirme,
seçim, katmanlar, web açma/kaydetme, büyük DWG), 6 niş istek (ViewCube sürükleme, tercihler, SVG
simgesi, büyüteç, kısıt uç durumu, macOS pencere denetimleri), 4 diğer (simge teklifi, ticari marka,
"Revitcraft", dinamik blok yardım teklifi). İlgi gerçek; açık sorunlar çekirdek yolla ilgili, bu da
indirimlerle örtüşüyor.

## Temel kullanıcı

Temel özelliklerde kalan sıradan bir kullanıcı: bir çizim başlatır, varsayılan ayarlarla yaygın çizim
ve düzenleme araçlarını kullanır, geri alır, kaydeder, PDF yazdırır. Hariç: gelişmiş seçenekler,
sayfa kümeleri, xref'ler, çizim stilleri, notlandırma stilleri, değişim uç durumları ve yukarıda
hariç tutulan her şey.

| Çekirdek özellik | Ağırlık | Derinlik | Ağırlıklı |
|---|---:|---|---:|
| Başlatma, Start sekmesi, yeni/aç çizim | 10% | 70% | 7.0 |
| LINE, PLINE, CIRCLE, ARC, RECTANG | 20% | 70% | 14.0 |
| MOVE, COPY, ROTATE, SCALE, MIRROR, ERASE | 15% | 70% | 10.5 |
| OFFSET, TRIM (hızlı), EXTEND, FILLET | 10% | 50% | 5.0 |
| Çalışan nesne yakalamaları, orto, kutupsal | 10% | 55% | 5.5 |
| Temel ölçüler (doğrusal, hizalı, yarıçap) | 8% | 60% | 4.8 |
| TEXT ve MTEXT | 5% | 55% | 2.8 |
| Katmanlar (oluştur, renk, aç/kapat) | 7% | 75% | 5.3 |
| Kaydırma, yakınlaştırma, seçme ve pencere seçimi | 5% | 65% | 3.3 |
| Geri al, yinele | 3% | 85% | 2.6 |
| Kendi çizimlerini kaydet ve yeniden aç | 4% | 75% | 3.0 |
| Yazdır veya PDF'e aktar | 3% | 45% | 1.4 |
| **Ortalama derinlik** | 100% | | **≈ 65%** |

| İndirim | Faktör | Kanıt |
|---|---:|---|
| Başlatma ve kararlılık | ×0.90 | Intel UHD grafiklerinde başlangıç çökmesi (#33, 0.4.0'da düzeltildi), çizgiler görünmüyor (#44), Linux fare gecikmesi (#25), web açma/kaydetme bozuk (#55, #133) |
| Keşfedilebilirlik ve arayüz netliği | ×0.92 | sağ tık menüleri yok (dinamik girdi #60'ta gerçek işaretçi kutularına kavuştu, bu bunu ×0.90'dan yükseltti), Yazdır'da çizdirme iletişim kutusu yok (#160), bazı menü öğeleri ancak yakın zamanda iletişim kutusu kazandı (#126) |
| İnsanların gönderdiği dosyaları açma | ×0.85 | başkalarının DWG'leri gerçek dosyalarda test edilmedi; 10 MB'lık bir DWG başarısız oldu (#57); Finder'dan `.dxf` başarısız (#158); 2007 öncesi CJK/Kiril DXF bozuk (kod sayfası yok) |

**Temel kullanıcı ≈ 65% × 0.90 × 0.92 × 0.85 ≈ 46%** (tahmini).

## Özellik alanına göre (özellikler boyutu)

Ağırlıklar tipik bir AutoCAD kullanıcısının çalışma süresinin payıdır (2B teknik ressamlar ve
detaylandırıcılar büyük çoğunluktur; 3B modelleme ve bulut özellikleri azınlıktır). Yüzdeler varlık
değil derinliktir: "not available yet" yazan bir seçenek sayılmaz.

| Alan | Ağırlık | Parite | Kalan (Opus 5.5 sa) | Kanıt ve ana eksikler | Belge |
|---|---:|---:|---|---|---|
| Çizim (2B) | 13% | 60% | 15–25 | ARC 11/11 yöntem, CIRCLE 6/6, RECTANG tüm seçenekler, REVCLOUD, WIPEOUT, XLINE; PLINE yay seçenekleri ve SPLINE seçenekleri tamam (#397, #408); MLINE, REGION ve HELIX nesnedir (#510); izometrik çizim (#556) | [eksikler](eksikler.md#özellikler) |
| Düzenleme | 15% | 50% | 40–60 | TRIM/EXTEND, OFFSET, ROTATE, PEDIT ve SPLINEDIT seçenekleri tamam (#397, #558, #559); ARRAYEDIT ile ilişkisel diziler (#448), dizi tutma noktaları yok; MATCHPROP seçici ve Settings (#439) | [eksikler](eksikler.md#özellikler) |
| Geometri hassasiyeti | 10% | 40% | 70–110 | çizgiler/yaylar/daireler için tam; elips ve spline kesişimleri, ofsetleri, en yakın noktaları mozaiklenmiş; mutlak toleranslar; poliline ofseti kabartıları yeniden hesaplamıyor | [geometry-parity](geometri-paritesi.md) |
| Katmanlar ve özellikler | 7% | 70% | 10–15 | filtreler, durumlar, VP geçersiz kılmalarıyla Katman Özellikleri Yöneticisi; QSELECT; Quick Properties; saydamlık; çizgi tipi yöneticisi zayıf; ~45 çizgi tipinden 19'u | [eksikler](eksikler.md#özellikler) |
| Ölçülendirme | 9% | 45% | 25–35 | tüm temel DIM türleri ve ilişkisellik; stil değişkenleri, DIMJOGGED, DIMJOGLINE, DIMBREAK, DIMEDIT, DIMINSPECT, QDIM modları (#507) ve TOLERANCE (#420); DIMSTYLE yöneticisi iletişim kutusu veya akıllı DIM yok | [eksikler](eksikler.md#özellikler) |
| Metin ve MTEXT | 6% | 35% | 30–45 | TrueType, biçimlendirme kodları, istifleme; MTEXT istem seçenekleri (#408), girintiler, sekmeler, madde işaretleri, numaralandırma ve sütunlar (#547), not ölçeklendirme (#468); düzenleyici iletişim kutusu, yerinde değil; alan veya SPELL yok (açık PR #551), SHX yok | [eksikler](eksikler.md#özellikler) |
| Kılavuzlar, tablolar, alanlar | 4% | 25% | 25–35 | MLEADER seçenekleri ve blok içeriği (#408), DXF'te yerel MULTILEADER (#459); hücre stilleri, formüller, bölme ve CSV veri bağlantılarıyla tablolar (#534); MLEADEREDIT/ALIGN/COLLECT yok, FIELD yok | [eksikler](eksikler.md#özellikler) |
| Tarama ve gradyanlar | 5% | 35% | 15–25 | adalarla nokta seçimi; ~85 desenden 23'ü; gradyanlar düz işleniyor; kaynak, boşluk toleransı, ayrı taramalar, ada stilleri ve HATCHEDIT seçenekleri (#451); Hatch and Gradient / Hatch Edit iletişim kutuları (Önizleme veya Inherit Properties yok) | [eksikler](eksikler.md#özellikler) |
| Bloklar, nitelikler, xref'ler, gruplar | 9% | 25% | 80–120 | statik bloklar ve nitelikler; ATTSYNC, BATTMAN düzenleme, DATAEXTRACTION/EATTEXT (#433); xref'ler (#525); gruplar (#417); statik bloklar için Blok Düzenleyici (BEDIT, BSAVE, BSAVEAS, BCLOSE); REFEDIT; dinamik blok başvuruları okunuyor ve düzenleniyor; dinamik blok yazarlığı, XCLIP, DesignCenter yok | [eksikler](eksikler.md#özellikler) |
| Yerleşimler, görünüm pencereleri, çizdirme | 9% | 35% | 35–50 | yerleşimler, VP başına katmanlar, sayfa kurulumları, çizim stilleri (CTB/STB), çizdirme iletişim kutusu (#160), çizdirme önizlemesi ve PUBLISH (#186), çizici aygıtları (#564), metin, katmanlar ve sayfalarla PDF (#567, #571), çokgen/nesne/kırpılmış görünüm pencereleri (#453), not ölçekleri (#468); MVIEW Restore yok, sistem yazıcıları yok | [eksikler](eksikler.md#özellikler) |
| Sorgu ve yardımcılar | 4% | 30% | 30–45 | DIST, AREA, ID, LIST, MEASUREGEOM, CAL, FIND, yerleşimler arası COUNT, QuickCalc, COMPARE (#563), bölgelerde MASSPROP (#510), otomatik kaydetme/kurtarma, AUDIT, RECOVER (#406), OPTIONS (#23); SPELL yok (açık PR #551), ETRANSMIT yok | [eksikler](eksikler.md#özellikler) |
| Parametrik kısıtlar | 2% | 50% | 20–30 | 12 geometrik kısıtın tümü ve boyutsal kısıtlar, kendi çözücümüz; DOF raporu, G2 Smooth, elips/spline/blok geometrisi ve CONSTRAINTINFER geldi (#546) | [geometry-parity](geometri-paritesi.md#kısıt-çözücü--50) |
| 3B modelleme, görselleştirme, işleme | 5% | 2% | 300–450 | UCS, 3B görünümler, yörünge, katılar, yüzeyler, ağlar, kesitler, görsel stiller, malzemeler, ışıklar veya işleme yok; 491 menü öğesinin 150'si | [eksikler](eksikler.md#özellikler) |
| İşbirliği ve bulut (Sheet Set Manager, Share, Traces, Markup Import, Activity Insights, DWG Compare) | 2% | 5% | 40–70 | bunların hiçbiri yok | [eksikler](eksikler.md#özellikler) |
| **Özellikler boyutu** | 100% | **≈ 41%** | **735–1,115** | | |

## Boyuta göre (gerçek işe hazır)

| Boyut | Ağırlık | Parite | Kalan (sa) | Tür | Belge |
|---|---:|---:|---|---|---|
| Özellikler (yukarıdaki tablo) | 35% | 41% | 735–1,115 | tahmini | bu dosya |
| Arayüz/UX sadakati (nokta girişi, yakalamalar, izleme, dinamik girdi, tutma noktaları, seçim, komut satırı, pencere çerçevesi) | 15% | 43% | 100–160 | tahmini, belgede ağırlıklı | [ui-parity](arayuz-paritesi.md) |
| Dosya biçimleri (önce DWG/DXF sadakati) | 20% | 30% | 170–280 | tahmini | [file-format-parity](dosya-bicimi-paritesi.md) |
| Kararlılık ve doğruluk | 10% | 35% | 30–50 | tahmini: yalnızca 2026-10-10'da yaklaşık 60 hata düzeltmesi birleştirildi, 30 açık sorun, `cargo fmt` main'de başarısız (PR #194), hiçbir CI, PR'larda Rust testlerini çalıştırmıyor | [eksikler](eksikler.md#kararlılık) |
| Performans | 5% | 60% | 20–40 | yalnızca iç sayılar (200k varlık akıcı); 1M varlık ve büyük DWG yüklemeleri test edilmedi; 10 MB'lık bir DWG başarısız olmak için ~2 dk sürdü (#57) | [hardware-parity](donanim-paritesi.md#donanım-üzerinde-performans-yalnızca-iç-sayılar) |
| Ekosistem ve otomasyon (AutoLISP/Visual LISP, .NET/ObjectARX, CUI, APPLOAD, Express Tools, App Store) | 5% | 10% | 120–200 | tahmini; MCP, CLI ve kontrol kanalı AutoCAD'in önünde, ancak LISP, CUI veya eklenti yok | [eksikler](eksikler.md#ekosistem-ve-otomasyon) |
| Donanım | 3% | 45% | 15–25 | tahmini | [hardware-parity](donanim-paritesi.md) |
| Platformlar | 3% | 75% | 15–25 | macOS, Windows (x64/x86/arm64), Linux, FreeBSD, web: AutoCAD'den fazlası; ancak web derlemesi dosyaları güvenilir açıp kaydedemiyor (#55, #133, #134) ve DWG yok, Windows/Linux çok az çalışma zamanı testi alıyor | [eksikler](eksikler.md#platformlar) |
| Yerelleştirme | 3% | 5% | 60–100 | ölçüldü: Ukraynaca dize kataloğu (936 satır, #36); Mac için AutoCAD'in 7 diğer dilinin hiçbiri | [localization-parity](yerellestirme-paritesi.md) |
| Yapay zekâ özellikleri (Autodesk Assistant, Smart Blocks, Markup Assist) | 1% | 20% | 40–80 | AutoCAD'in uygulama içi yapay zekâsının hiçbiri; MCP üzerinden ajan kontrolü farklı bir güç | [eksikler](eksikler.md#yapay-zekâ-özellikleri) |
| **Gerçek işe hazır** | 100% | **≈ 38%** | **≈ 1,300–2,100** | | |

Boyut ağırlıkları, bir profesyonelin geçip geçemeyeceğine neyin karar verdiğini tanımlar: özellikler
ve dosya uyumluluğu baskındır.

## Kalan emek ve nasıl kalibre edildiği

**Kalibrasyon.** SPCAD'i inşa eden oturumun Claude Code transkriptleri, ilk sürümünün tamamının
(yaklaşık 51,000 satır, 290 komut, 47% menü genişliği) **yaklaşık 11.5 Opus 5.5 ajan-saati**
sürdüğünü gösteriyor: 6.9 saat baş ajan (2026-10-07 02:16–09:08 UTC) artı sekiz alt ajanın yaklaşık
4.6 saati, bir kısmı paralel. (Git commit tarihleri o oturumdan daha önceye yeniden yazıldı ve
kullanılamaz.) Bu, **varlık kalitesinde** ajan-saati başına yaklaşık 25 komuttur. Derinlik çok daha
pahalıdır: ertesi gün dış katkıcılar yaklaşık 60 doğruluk hatası bulup düzeltti, her biri gerileme
testli küçük bir PR ve bu geçişteki denetimler yalnızca "not available yet" yazan düzinelerce istem
seçeneği buldu. Burada kullanılan oranlar: bir seçenek veya hata düzeltmesi 0.3–0.7 sa; orta ölçekli
bir özellik (nesne yakalama izleme, düzenlenebilir dinamik girdi, bir iletişim kutusu) 2–5 sa; bir alt
sistem (xref'ler, dinamik bloklar, AutoLISP, çizim stilleri) 15–100 sa; 3B çekirdek 300–450 sa; sonra
AutoCAD'e karşı doğrulama için 1.3× kuyruk faktörü.

**Toplam neden yükseldi.** 2026-10-07 yol haritası tam pariteyi ajan bildirimli kilometre taşı
yüzdelerinden ≈ 29% ve ≈ 570 sa kalanla koymuştu. Bu geçiş varlık yerine davranışı denetledi ve yeni
kanıt buldu: nesne yakalama izleme, uzantı, paralel ve görünen yakalamalar anahtardır; dinamik girdi
salt okunurdur; kaydetmeler okuyucunun modellemediği her varlığı düşürür; DWG her zaman R2000 olarak
kaydedilir; toleranslar mutlaktır; ve eski tahminde yerelleştirme, ekosistem, platformlar, kararlılık
veya yapay zekâ için satır yoktu. Ağırlıklı rakam yükseldi (29% → 41% özellikler) çünkü 2026-10-07'den
beri hem genişlik hem derinlik büyüdü ve ağırlıklar artık yazılı; saatler yükseldi çünkü kalan iş
derinlik ve 3B'dir.

**Paralellik.** İşin yaklaşık 70%'i alana göre 4–6 ajan arasında bölünür (her özellik alanı, dosya
biçimleri, arayüz hassasiyeti, dil başına yerelleştirme), bu yüzden geçen süre ajan-saatlerinin
kabaca dörtte biridir. İnsan gerektirir: her çevirinin ana dil konuşuru incelemesi; lisanslı bir
gerçek dosya DWG/DXF korpusu (dosyaların sahiplerinden); kaydettiklerimizin AutoCAD'de kara kutu
kontrolleri; 3D fare ve tablet donanımı; eklenti barındırma ve DGN hakkında sahip kararı.

## Bu değerlendirmenin geçmişi

| Tarih | Özellik genişliği | Ağırlıklı parite | Kalan | Not |
|---|---|---|---|---|
| 2026-10-10 (birleştirme) | değişmedi (233/491 yeniden tarandı) | hazır ≈ 38% (37.7%), ana akım ≈ 28%, temel ≈ 46% | yuvarlama içinde değişmedi | main, dinamik girdi işaretçi kutularını (#60), Ukraynaca kataloğu (#36), WIPEOUT/gradyan DXF düzeltmelerini (#147, #162) birleştirdi: arayüz/UX 40% → 43%, yerelleştirme 3% → 5% |
| 2026-10-10 (sonra) | değişmedi | gerçek işe hazır ≈ 37% (değişmedi: zaten yazılı boyut ağırlıklarından inşa edildi, gizli yargı yok); ana akım uygulayıcı ≈ 27%; temel kullanıcı ≈ 45% | değişmedi | ana akım ve temel sayılar eklendi (craftrules standardı) |
| 2026-10-10 | 245/491 (50%), yalnızca motor 233/491 | özellikler ≈ 41%, gerçek işe hazır ≈ 37% | 1,300–2,100 sa | bu biçimde tam yeniden ölçüm (bu dosya) |
| 2026-10-07 | 233/491 (47%) | genel ≈ 29%, 2B ≈ 55% | ≈ 570 sa | eski ROADMAP: inşa eden ajanların bildirdiği kilometre taşı yüzdeleri |

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-12 | küçük | Tarama satırı kanıtı: Hatch and Gradient / Hatch Edit iletişim kutuları; yüzdeler ve saatler yeniden ölçülmedi |
| 2026-10-12 | küçük | Bloklar satırı kanıtı: statik bloklar için Blok Düzenleyici (BEDIT, BSAVE, BSAVEAS, BCLOSE); yüzdeler ve saatler yeniden ölçülmedi |
| 2026-10-11 | küçük | Birleştirilen iş için özellik alanı satırlarının kanıt metni yenilendi (#397, #406, #408, #417, #420, #433, #448, #451, #453, #459, #468, #507, #510, #525, #534, #546, #547, #558, #563, #564, #567, #571); parite yüzdeleri ve saatler yeniden ölçülmedi |
| 2026-10-10 | küçük | Main birleştirildi: dinamik girdi işaretçi kutuları (#60) arayüz/UX'u 43%'e, Ukraynaca katalog (#36) yerelleştirmeyi 5%'e yükseltti; hazır 37% → 38%, ana akım 27% → 28%, temel 45% → 46% |
| 2026-10-10 | küçük | Hedef kitleye göre hazırlık tablosu: tam (1,150–1,850 sa), ana akım (570–880 sa) ve temel (140–225 sa) için ~95%'e saatler; tam sayının toplamsal ağırlıklı bir toplam olduğu doğrulandı, değişmedi |
| 2026-10-10 | küçük | Ana akım uygulayıcı (≈ 27%) ve temel kullanıcı (≈ 45%) sayıları, yazılı ağırlıklar, indirimler ve kullanıcı kanıtıyla |
| 2026-10-10 | küçük | Aşama, çekirdek iş akışı kapısı altında alfa → alfa öncesi; alfaya kalan satır |
| 2026-10-10 | büyük | Eski ROADMAP'in parite bölümünden oluşturuldu; ağırlıklar, iki sayı, alan başına saatler ve derleme oturumunun transkriptlerinden kalibrasyonla tam yeniden ölçüm |
