# SPCAD denetim protokolü

Masaüstü uygulamasını `--control PORT` (veya `SPCAD_CONTROL_PORT=PORT`) ile başlatın. `127.0.0.1:PORT`
üzerinde JSON satırlarını dinler: satır başına bir istek, satır başına bir yanıt. `--control` ile verilen
bir port değişkendekini geçersiz kılar; değer verilmeden kullanılan `--control` değişkenin portunu korur.

```json
{"id": 1, "method": "cmdline.input", "params": {"text": "circle 0,0 5"}}
{"id": 1, "ok": true, "result": {"prompt": "Command:", "output": ["Command: CIRCLE"], ...}}
```

Hatalar `{"id": …, "ok": false, "error": "message"}` biçiminde döner. Bilinmeyen bir yöntemin hatası,
sunucunun yanıtladığı yöntemleri listeleyen `control.methods`'a işaret eder.

Dize türünde bir `method` içeren bir JSON nesnesi olmayan satır (bir HTTP isteği, JSON olmayan metin,
UTF-8 olmayan baytlar) tek bir hata yanıtı alır ve sunucu bağlantıyı kapatır; ondan sonra gönderilen
hiçbir şey çalışmaz. Boş satırlar atlanır.

Sunucu, bir istemcinin neleri tutabileceğini sınırlar: aynı anda 16 bağlantı (fazlası bir hata satırı alır ve
kapatılır), istek satırı başına 256 MiB (daha uzunu bağlantıyı kapatır), 64 sıraya alınmış istek (fazlası
`{"ok": false, "error": "busy: …", "state": "not-run"}` ile yanıtlanır) ve soket kapatılmadan önce 10
dakika boşta bekleme veya engellenen yanıt yazımı başına 30 saniye.

Sunucu, uygulamanın bir isteği yanıtlaması için en fazla 60 sn bekler. Bundan sonra zaman aşımıyla yanıtlar
ve `state`, isteği yeniden göndermenin güvenli olup olmadığını söyler:

- `{"id": …, "ok": false, "error": "timeout", "state": "not-run"}`: uygulama isteği başlatmamıştı.
  İptal edilir ve asla çalışmaz, bu yüzden yeniden göndermek güvenlidir.
- `{"id": …, "ok": false, "error": "timeout", "state": "may-have-run"}`: uygulama onu çoktan başlatmıştı.
  Yine de tamamlanıp çizimi değiştirebilir ve geç gelen sonucu atılır. Yeniden göndermeden önce çizimi
  inceleyin (`drawing.inspect`).

Aşağıdaki tablo, üst düzey denetim protokolü yöntemlerini açıklar: isteğin `method` alanı için adlar.
`engine.commands` ek yöntemlerin listesini değil, ayrı bir komut kataloğu döndürür.

| Yöntem | Parametreler | Ne yapar |
|---|---|---|
| `cmdline.input` | `{text}` | Komut satırına tıpkı bir kullanıcı gibi bir satır yazın. Komutları ad/takma ad ile başlatır ve istemleri yanıtlar (`x,y` noktaları, `@dx,dy`, `@d<a`, mesafeler, anahtar sözcükler, boş = Enter). Boşluklar metin istemleri dışında Enter gibi davranır. |
| `cmdline.script` | `{text}` | Çok satırlı bir betik çalıştırın (`.scr` dosyaları gibi). |
| `cmdline.key` | `{key: "enter"\|"escape"}` | Enter veya Escape'e basın. `cmdline.input` gibi yanıtlar: durum, `output` ve Enter reddedildiğinde `error`. |
| `cmdline.state` | | Geçerli istem, anahtar sözcükler, kabul edilen girdi türleri, geçmişin sonu. |
| `engine.execute` | `{command, params}` | Herhangi bir komutu JSON parametreleriyle çalıştırın. Asla bir iletişim kutusu açmaz. `command`, aynı parametreler ve sonuçla bu yöntemin bir takma adıdır. |
| `engine.commands` | | Her komut: kimlik, etiket, menü yolu, kısayol, takma adlar, parametre belgesi, etkin. |
| `drawing.inspect` | `{entities?, limit?}` | Çizim özeti (sayımlar, sınırlar, katmanlar, stiller, bloklar, yerleşimler, seçim, geri al) ve tanıtıcıları ve geometrisiyle varlıklar. |
| `ui.inspect` | | Arayüz durumu, tuval dikdörtgeni, görünüm, imleç ve yakalama, performans sayaçları; `closePrompt`: bir "Save changes?" isteminin sorduğu çizim (yoksa null); ve gerçek egui klavye odağı (`focus.id`, `focus.label`, `focus.role`). |
| `ui.elements` | `{label?, role?, focused?}` | En son arayüz karesinden gözlenen araç çubuğu, sekme, çizim ve katman paleti denetimleri. Filtreler tam olarak eşleşir. Her girdi gerçek egui `id`'sini, etiketini, rolünü, `[x,y,width,height]` dikdörtgenini, etkin/görünür/seçili durumunu ve geçerli odağını taşır. |
| `ui.menu.list` / `ui.menu.invoke` | `{command}` | Menü ağacı; bir öğeyi tıklama gibi çağırın (etkileşimli). |
| `ui.pointer` | `{x, y, space?: "world"\|"screen", button?, action?: "click"\|"move"}` | Çizim alanında tıklayın (varsayılan olarak dünya koordinatları). Sağ düğme = Enter. |
| `ui.click` / `ui.move` | `{x, y, button?, shift?, ctrl?}` | Ekran noktalarında gerçek egui işaretçi olayları (her pencere öğesine ulaşır). |
| `ui.drag` | `{x, y, to: [x, y], button?}` | Gerçek basma, taşıma ve bırakma (komut satırının üst kenarını, ViewCube halkasını sürükleyin…). |
| `ui.key` / `ui.text` | `{key, cmd?, shift?, alt?}` / `{text}` | Sentetik klavye girdisi. |
| `ui.set` | UiState alanları | Paletleri, araç çubuklarını, komut satırını, ViewCube'ü gösterin/gizleyin…; `theme: "system"\|"light"\|"dark"` (ayrıca `engine.execute` ile `ui.theme {theme}`); `ui.inspect` gösterilen temayı bildirir (`theme`: light/dark). `animations: "system"\|"on"\|"off"` (ayrıca `ui.animations {animations}`) animasyon efektlerini (iletişim kutusu ve menü solmaları, yumuşatılmış kaydırma) gösterir veya gizler; `system` işletim sisteminin ayarını izler; `ui.inspect` bunların gösterilip gösterilmediğini bildirir (`animations`: true/false). `saveFormat: "dxf"\|"dwg"` (ayrıca `ui.saveformat {format}`) Farklı Kaydet'in yeni çizimler için önerdiği biçimdir; `saveVersion: "2000"`…`"2018"` (ayrıca `ui.saveversion {version}`) bunları kaydettiği sürümdür (SAVEAS ayrıca doğrudan `version` ve `binary` alır). |
| `ui.resize` | `{width, height}` | Pencereyi yeniden boyutlandırın. |
| `ui.focus` | | Yerel pencereyi etkinleştirin. Bir pencere öğesine odak atamaz. |
| `ui.screenshot` | `{path?}` | Pencerenin PNG'si (sunulmuş bir kare gerektirir). |
| `ui.render` | `{path?, width?, height?, fit?}` | Başsız işleme. `fit` (varsayılan true) tüm çizimi çerçeveler; `false` geçerli görünümü kullanır. `{pngBase64, width, height, bytes, path?}` yanıtlar. |
| `app.open` / `app.save` / `app.quit` | `{path}` | Dosya işlemleri. |
| `control.methods` | | Bu sunucunun yanıtladığı her yöntem: `{method, params, description}`. Başsız (`spcad-cli mcp`) yalnızca hizmet ettiği yöntemleri listeler. |
| `ping` | | `"pong"` yanıtlar. Ucuz bir sağlık kontrolü. |

`engine.commands` tarafından listelenen bir komutu çalıştırmak için kimliğini `params.command` içine,
argümanlarını iç içe `params.params` nesnesine koyun ve üst düzey yöntem olarak `engine.execute` kullanın.
Bu, `open` ve `document.bytes` ile masaüstü uygulamasının kataloğundaki arayüz komutlarını içerir.
`drawing.inspect` de bir motor komutudur, ancak açık bir üst düzey yöntemi vardır; noktalı bir komut kimliği
tek başına onu bir protokol yöntemi yapmaz.

Örneğin, çizimi inceleyin ve komutları doğrudan listeleyin, ardından bir çizim açın ve DXF baytlarını
`engine.execute` aracılığıyla alın (satır başına bir istek):

```json
{"id":2,"method":"drawing.inspect","params":{}}
{"id":3,"method":"engine.commands","params":{}}
{"id":4,"method":"engine.execute","params":{"command":"open","params":{"path":"ornekler/apartment.dxf"}}}
{"id":5,"method":"engine.execute","params":{"command":"document.bytes","params":{}}}
```

`open` örneği, uygulamanın depo kökünden başlatıldığını varsayar; aksi halde uygulamanın erişebildiği
mevcut bir çizime giden bir yol kullanın. `document.bytes` varsayılan olarak DXF'tir ve base64 ile
kodlanmış baytları sonucun `data` alanında döndürür. `command` takma adı aynı şekilde kullanılabilir:

```json
{"id":6,"method":"command","params":{"command":"document.bytes","params":{}}}
```

`open` veya `document.bytes` üst düzey `method` olarak gönderildiğinde bir `unknown method` hatası döner:
protokol yöntemleri açıkça yönlendirir ve komut kayıt defterine düşmez. `app.open`, `{path}` ile bir çizim
açmak için üst düzey bir kolaylık yöntemidir.

Seçili nesneleri tek bir sorguda yapılandırılmış geometri olarak okumak için `entities` komutunu
`selected: true` ile çalıştırın (bir boolean; atlanırsa/false ise filtresiz seçim davranışı korunur):

```json
{"id":7,"method":"engine.execute","params":{"command":"entities","params":{"selected":true,"limit":1000}}}
```

Yalnızca etkin uzaydaki seçili varlıklar döner; boş bir seçim boş bir sayfa döndürür. Seçim filtresi
`offset`/`limit` öncesinde `type`, `layer` ve `window` ile birleşir. Mevcut yanıt değişmez: `count` tüm
eşleşmelerdir, `returned` sayfa boyutudur ve `entities` tanıtıcıları, katmanları ve geometriyi içerir. Bu
sorgu seçimi, çizim içeriğini veya geri alma geçmişini değiştirmez. `entities` bir komuttur, üst düzey bir
protokol yöntemi değildir.

MCP sunucusu (`spcad-cli mcp --connect 127.0.0.1:PORT`) bu protokolü sarar; başsız (`spcad-cli mcp`) aynı
yöntemleri süreç içi bir oturumdan sunar. MCP araç adları başka bir arayüzdür: `execute` `engine.execute`'a,
`list_commands` `engine.commands`'a ve MCP `open` aracı `app.open`'a eşlenir. Bkz. [mcp.md](mcp.md).

`ui.key` gerçek egui tuş olaylarını kuyruğa alır; yanıtı kuyruğu onaylar. Bir sonraki eylemi göndermeden önce
ortaya çıkan odak veya durum için `ui.inspect` ya da `ui.elements` yoklayın. Denetimler arasında gezinmek için
Tab ve Shift-Tab, etkinleştirmek için Enter/Boşluk kullanın. Gözlenen kayıt defteri işletim sisteminin
erişilebilirlik ağacı değildir ve fiziksel girdi veya ekran okuyucu kabulünü ortaya koymaz. Kayıtlı olmayan
denetimler yine de gerçek odak kimliklerini null bir etiket/rol ile bildirir.

Yerel ana bilgisayar yerleşim seçimlerini `workspace.json` dosyasından geri yükler ve değişiklikleri atomik
olarak kaydeder. Palet/araç seti görünürlüğünü, seçili araç setini, daraltılmış grupları, komut satırı geçmişi
yüksekliğini ve ilgili pencere çerçevesi seçimlerini saklar. Çizimleri, komut tamponlarını, iletişim kutularını
veya geçerli Start sekmesini saklamaz. Varsayılan dizin macOS'ta `~/Library/Application Support/SPCAD`,
Windows'ta `%APPDATA%/SPCAD`, diğerlerinde `$XDG_CONFIG_HOME/spcad` veya `~/.config/spcad`'dir. Yalıtılmış bir
profil için `SPCAD_CONFIG_DIR` değerini özel bir dizine ayarlayın; açıkça boş bir değer çalışma alanı
okumalarını ve yazmalarını devre dışı bırakır. Geçersiz, aşırı büyük veya daha yeni sürümlü bir dosya bildirilir
ve dokunulmadan bırakılır; o başlatma için kalıcılık devre dışı kalır. Çalışma alanı seçimlerini kaydetmeye
devam etmek için yeniden başlatmadan önce o dosyayı onarın veya taşıyın.

## Panel kenetleme

Araç Takımları, Katmanlar ve Özellikler yeniden gruplanabilir, bölünebilir, uygulama penceresi içinde yüzdürülebilir, kapatılabilir ve yeniden açılabilir. Çizim tuvali görünür kalır ve kapatılamaz veya bir sekmeye dönüştürülemez. Belge/oturum görünümü korunur. Yerleşimler, etkin sekmeler, panel görünürlüğü ve yüzen dikdörtgenler uygulamanın mevcut arayüz tercihleriyle kaydedilir. Bu işlemler belge içeriğini düzenlemez.

Panel kimlikleri: `shelf` (Araç Rafı), `layers`, `properties`, `externalReferences`. Korumalı merkez `canvas`'tir.

`ui.dock` bir `operation` kabul eder; temel panel başına işlemler ayrıca `panel` gerektirir. Aşağıdaki gelişmiş `resizeSplit` işlemi panel yerine bölünmüş bir `path` kullanır. Temel işlemler `open`, `close`, `activate`, `float` (sonlu pozitif `rect: [x, y, width, height]` ile) veya `move` (`target` ve `zone: center|left|right|top|bottom` ile) şeklindedir. Merkez, hedefle gruplanır; yan bölgeler onun yanında bölünür. `ui.dock.reset` varsayılan düzeni geri yükler. Geçersiz taşımalar düzeni değiştirmeden bir hata döndürür.

QuickCalc paleti (`quickCalc`) varsayılan düzenin parçası değildir: QUICKCALC (`ui.panel.quickcalc`, Cmd+8) onu açar ve ilk seferde yüzdürür.

Bu arayüz komutlarını `engine.execute` aracılığıyla çağırın, örneğin `{"command":"ui.dock","params":{"operation":"float","panel":"layers","rect":[100,100,320,440]}}`.

Gelişmiş `ui.dock` işlemleri, işaretçi hareketleriyle aynı doğrulanmış işlemi kullanır:
`moveFloating` `rect`i günceller, `resizeSplit` bir boolean alt `path` ve
`size` (`{"Ratio":0.5}`, `{"FixedFirst":280}` veya `{"FixedSecond":280}`) kabul eder,
`setStackOpen` `open` alır ve `resizeStack` bir nokta `height` ya da null alır.
Bir merkez `move`, sekme sırası için `before` (panel kimliği veya sona eklemek için null) içerebilir.
Eşdeğer `{"action": ...}` biçimi, serileştirilmiş paylaşılan bir kenetleme eylemi kabul eder;
bu aynı zamanda görüntüleyicinin yaydığı biçimdir. Korumalı tuval ve uygulamaya özgü panel
kuralları her iki biçim için de eşit şekilde geçerlidir.

## Komut satırı yanıtları

Uygulama ve başsız oturum aynı alanlarla yanıtlar.

`cmdline.state`, `cmdline.key` ve `cmdline.input` komut satırı durumunu döndürür:

| Alan | Değer |
|---|---|
| `prompt` | İstem metni (boştayken `"Command:"`). |
| `running` | Çalışan komutun kimliği veya `null`. |
| `keywords` | Geçerli istemin seçenekleri. |
| `accept` | Geçerli istemin kabul ettiği girdi türleri veya boştayken `null`. |
| `buffer` | Komut satırına yazılmış ama henüz gönderilmemiş metin (başsızda her zaman `""`). |
| `history` | Son 20 geçmiş satırı. |
| `historyExpanded` | Geçmişin genişletilip genişletilmediği (başsızda her zaman `false`). |

`cmdline.input` şunları ekler:

- `output`: girdinin eklediği geçmiş satırları. Satır reddedildiğinde hata iletisini de içerir.
- `error`: hata iletisi, yalnızca satır reddedildiğinde (bilinmeyen komut, geçersiz girdi…).

Reddedilen bir satır yine de `"ok": true` yanıtlar, çünkü metin yazılmıştır. `error` alanını kontrol edin.

Dize türünde bir `path` olmadan `app.open`, herhangi bir komut çalışmadan `"missing path"` ile başarısız olur.

## Arayüz dili

Bir dili `engine.execute` ve `ui.language` arayüz komutuyla seçin:

```json
{"id":1,"method":"engine.execute","params":{"command":"ui.language","params":{"lang":"uk"}}}
```

Desteklenen değerler `auto`, `en`, `uk`, `pt-BR`, `de`, `es`, `cs`, `ru`, `fr`, `zh`, `it`, `ko`, `sv`, `ar` ve `hu`'dur.
Çince kurulu bir CJK yazı tipi, Arapça kurulu bir Arapça yazı tipi gerektirir; Korece, isteğe bağlı craft-fonts
girdisinden veya işletim sisteminden Korece arayüzü kapsayan bir yazı tipi gerektirir. Uygun bir yazı tipi
olmadan kaydedilen seçim korunur ve sonucun `shown` değeri `en` olur. `ui.set` ayrıca
`{"interfaceLanguage":"uk"}` kabul eder; `ui.inspect` kaydedilen seçimi arayüz durumunda bildirir.
Geçersiz seçimler geçerli dili değiştirmez. Denetim kanalının döndürdüğü menüler, komut etiketleri ve istem
metni İngilizce kaynak metinlerini korur. Bkz. [yerellestirme.md](yerellestirme.md).
