# SPCAD ve yapay zekâ ajanları (MCP)

SPCAD, [Model Context Protocol](https://modelcontextprotocol.io) protokolünü stdio üzerinden konuşur.

## Başlatma

```sh
# Başsız: süreç içi bir çizim oturumu, pencere yok.
spcad-cli mcp

# Çalışan uygulamaya köprülenmiş (ajanın çizmesini izleyin):
spcad --control 7979 &
spcad-cli mcp --connect 127.0.0.1:7979
```

Bir MCP istemcisi genellikle sunucularını kendisiyle birlikte başlatır, çoğu zaman uygulama açılmadan önce.
`--lazy` ile köprü yine de başlar ve uygulamaya ilk araç çağrısında ulaşır; uygulama yanıtlayana kadar
her çağrı nasıl açılacağını söyleyen bir iletiyle başarısız olur. `--launch` ayrıca, hiçbir şey bulamayan
bir çağrıda SPCAD'i (macOS) o denetim portuyla açar ve onu bekler:

```json
{ "mcpServers": { "spcad": { "command": "spcad-cli", "args": ["mcp", "--connect", "127.0.0.1:7979", "--launch"] } } }
```

`--connect`, uygulamanın `HOST:PORT` bilgisini gerektirir. Bu olmadan (veya bilinmeyen bir argümanla)
`spcad-cli mcp`, başsız bir oturuma geri dönmek yerine bir hatayla çıkar; böylece uygulama için
tasarlanmış bir istemci yanlışlıkla ayrı bir çizimi asla düzenlemez. Uygulamaya ulaşılamazsa başlatma da
başarısız olur.

Örnek istemci yapılandırması (Claude Code / Claude Desktop):

```json
{ "mcpServers": { "spcad": { "command": "spcad-cli", "args": ["mcp"] } } }
```

## Araçlar

| Araç | Ne yapar |
|---|---|
| `command_line {text}` | Komut satırına tıpkı bir kişi gibi yazın: `line 0,0 @10,0 @0,5 c`, `circle 5,2 1`, `offset 0.5`, `zoom e`. Boşluklar Enter gibi davranır; boş metin Enter'dır. Yeni istemi, anahtar sözcükleri ve çıktıyı döndürür. |
| `script {text}` | Çok satırlı bir betik (`.scr` dosyaları gibi). |
| `execute {command, params}` | JSON parametreleriyle herhangi bir komut, asla bir iletişim kutusu. `list_commands` kimlikleri ve parametre belgelerini gösterir. |
| `list_commands {filter?}` | Komut kataloğu; `filter` bir komutun kimliğinin, etiketinin veya bir takma adının bir kısmıyla eşleşir. |
| `inspect_drawing {entities?, limit?}` | Sayımlar, sınırlar, katmanlar, stiller, bloklar, yerleşimler, seçim, geri alma geçmişi ve varlıklar. |
| `query_entities {type?, layer?, window?, crossing?, limit?, offset?}` | Tanıtıcıları ve geometrisiyle filtrelenmiş varlıklar. `window` `[[x0,y0],[x1,y1]]`, geometrisi içinde olan veya onu kesen nesneleri bulur (`crossing: false`: tamamen içinde). `count` her eşleşmedir, `returned` sayfadır (`offset`, `limit`, varsayılan 500). |
| `execute {"command": "shapes", "params": {type?, layer?, window?, crossing?, limit?, offset?, tolerance?}}` | Kapalı şekiller hakkında bilgiler — kapalı çoklu çizgiler, daireler, tam elipsler, kapalı spline'lar: çevre, alan, ağırlık merkezi, dolanım yönü, ilk kendi kendini kesişme ve çakışık köşeler; yaylar ve şişkinliklerle birlikte tam. Diğer eşleşmeler, nedenleriyle birlikte `skipped` altında sayılır. |
| `query_entities {type?, layer?, selected?, window?, crossing?, limit?, offset?}` | Tanıtıcıları ve geometrisiyle filtrelenmiş varlıklar. `selected: true` sonuçları etkin uzaydaki geçerli seçimle sınırlar; boş bir seçim hiçbir varlık döndürmez. Atlanırsa/false ise o uzaydaki tüm varlıklar sorgulanır. Filtreler sayfalamadan önce birleşir. `window` `[[x0,y0],[x1,y1]]`, geometrisi içinde olan veya onu kesen nesneleri bulur (`crossing: false`: tamamen içinde). `count` her eşleşmedir, `returned` sayfadır (`offset`, `limit`, varsayılan 500). |
| `render {width?, height?, fit?, window?}` | Çizimin PNG görüntüsü veya yalnızca `window` `[[x0,y0],[x1,y1]]` alanının görüntüsü (büyük bir çizimin tek bir sayfası veya odası; boyutlandırılmadıkça pencerenin şekliyle 1600 px genişliğinde). |
| `find_text {contains?, layer?, window?, limit?, offset?}` | Okunabilir metinler — TEXT, biçimlendirme kodları olmadan MTEXT, blok nitelikleri, çoklu kılavuzlar — metin, ekleme noktası, yükseklik, döndürme, stil, katman ve tanıtıcı ile; içeriğe (her durumda), katmana ve ekleme noktalarının etrafındaki bir pencereye göre filtrelenir. Salt okunur (`texts` komutu). |
| `new_drawing {metric?}`, `open {path}`, `save {path?}`, `cancel` | Dosyalar ve Escape. `.svg`, `.png` veya `.pdf` yoluna `save` dışa aktarır: çizim adını ve kaydedilmemiş değişikliklerini korur. |
| `screenshot`, `ui_inspect`, `ui_click {x, y}` | Yalnızca uygulamaya bağlıyken. |

Kaynaklar: `spcad://drawing` (inceleme JSON'u) ve `spcad://commands` (katalog).

Argümanlar her aracın girdi şemasına göre denetlenir. Bilinmeyen bir araç, nesne olmayan `arguments`, eksik
zorunlu argüman veya yanlış türde bir argüman bir JSON-RPC hatası `-32602` (invalid params) olur; böylece
`command_line {}` asla yanlışlıkla Enter'a basmaz. Çalışıp başarısız olan bir araç (kötü bir nokta,
bilinmeyen bir komut) `isError: true` ve iletiyle bir sonuç döndürür.

## Tipik bir oturum

1. `command_line {"text": "rectang 0,0 100,60"}`
2. `execute {"command": "circle", "params": {"center": [10, 10], "radius": 4}}`
3. `command_line {"text": "dimlinear 0,0 100,0 50,-8"}`
4. `inspect_drawing {}` — sayımları ve geometriyi doğrulayın.
5. `render {}` — sonuca bakın.

`kitapliklar/mcp/src/testler.rs` içindeki uçtan uca test böyle bir oturum çalıştırır ve sonucu yalnızca MCP
aracılığıyla denetler.
