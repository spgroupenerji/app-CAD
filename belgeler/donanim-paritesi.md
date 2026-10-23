# Donanım paritesi

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-13 · **Değişiklik:** küçük (yakınlaştırma ve kaydırma arka planda yeniden oluşturulur) · **Hedef:** Autodesk AutoCAD 2027 (Mac ve Windows)

AutoCAD'in kullandığı donanım ve SPCAD'nin platform başına nasıl karşılaştırıldığı. İş maddeleri:
[eksikler.md](eksikler.md#donanım).

**Özet (tahmini): donanım ≈ 45%.** 2B GPU çizimi, HiDPI ve izleme dörtgenleri her platformda
kapsanıyor. Eksik: 3D fareler, tabletler/dijitalleştiriciler, GPU hızlandırmalı 3B (3B yok) ve
kasıtlı herhangi bir çoklu monitör işleme. Kalan: **≈ 15–25 Opus 5.5 saat**, ayrıca 3D fareleri ve
tabletleri test etmek için cihazlara sahip bir insan.

2026-10-10'da kaynaktan ölçüldü (`kitapliklar/arayuz-egui/src/gpu.rs`, `canvas.rs`,
`uygulamalar/spcad/src/grafikler.rs`, `uygulamalar/spcad-web`); AutoCAD tarafı Autodesk'in sistem
gereksinimlerinden ve grafik performansı belgelerinden. Bu geçişte hiçbir kıyaslama yapılmadı.

| Özellik | AutoCAD 2027 | SPCAD macOS | SPCAD Windows | SPCAD Linux / FreeBSD | SPCAD web |
|---|---|---|---|---|---|
| 2B için GPU tuvali | evet (donanım hızlandırma, GPU yumuşatma, yüksek kaliteli geometri) | Metal üzerinde wgpu | varsayılan olarak DirectX 12 üzerinde wgpu (`WGPU_BACKEND` geçersiz kılar; AMD çökmelerinden sonra GL'den kaçınılır, #31) | Vulkan (veya GL) üzerinde wgpu | WebGPU, WebGL2 geri dönüşü |
| CPU geri dönüşü | yazılım kipi | evet (`tiny-skia` rasterleştirici) | evet | evet | — |
| 3B için GPU (gölgeli görsel stiller, ışın izlemeli oluşturma) | evet | hayır (3B yok) | hayır | hayır | hayır |
| Düşük gecikmeli sunum | evet | `SPCAD_VSYNC` ile vsync veya düşük gecikme | aynı | vsync kuyruğu yok, böylece artı işareti yetişir (#183) | tarayıcı kontrollü |
| HiDPI / Retina, ekran ölçekleme | evet | evet (piksel başına egui noktası) | evet; bir Windows 11 DPI düzeltmesi upstream'de kuyrukta (PR #110) | evet | evet |
| Çoklu monitör (diğer ekranlarda paletler, monitör başına DPI) | evet | kasıtlı olarak işlenmez; tek pencere | aynı | aynı | — |
| İzleme dörtgeni kaydırma, sıkıştırarak yakınlaştırma | evet (Mac) | evet, izleme dörtgeni ile tekerlek ayrımıyla (#58) | yalnızca tekerlek (hassas dokunmatik yüzey test edilmedi) | tekerlek; Linux Mint'te gecikme bildirildi (#25) | tarayıcı olayları |
| Magic Mouse | evet | izleme dörtgeni olarak ele alınır | — | — | — |
| Tekerlekli fare, orta tuşla kaydırma | evet | evet | evet | evet | evet |
| 3D fare (3Dconnexion SpaceMouse) | evet | **hayır** | **hayır** | **hayır** | hayır |
| Kalem tableti / dijitalleştirici (TABLET komutu, kalibrasyon) | evet (Windows) | **hayır** | **hayır** | **hayır** | hayır |
| İmleci izleyen ekran büyüteci | evet | — | bozulur (#39; PR #187 bir sistem imleci seçeneği ekler) | — | — |
| Çiziciler ve yazıcılar (PC3 aygıtları, sistem yazıcıları) | evet | **hayır**: yalnızca PDF/PNG/SVG dosyaları; Print PDF'i nereye kaydedeceğini sorar (#26) | aynı | aynı | indirme yerine PDF baytları gösterilir (#134) |

## Donanım üzerinde performans (yalnızca iç sayılar)

Önceki yol haritasından (onu inşa eden ajanlar tarafından ölçüldü, 2026-10-06, `spcad-cli perf`):
200,000 varlıkta seçim 0.001 ms, bir düzenlemeden sonra yaklaşık 4 ms (artımlı R-ağacı) ve GPU
tuvali bir kareyi yaklaşık 4 ms'de çizer. Aynı makinede AutoCAD ile hiçbir şey karşılaştırılmadı
ve 1M varlıklı çizimler test edilmedi. 10 MB'lık bir DWG'nin başarısız olması yaklaşık iki dakika
sürdü (#57).

Yakınlaştırma ve kaydırma artık pencereyi dondurmuyor (2026-10-13). Taranmış 12k varlıklı bir
mimarın DWG'sinde (yaklaşık 340k görüntüleme ilkeli) her yakınlaştırma adımı, görüntüleme listesi,
köşe verisi ve GPU arabellekleri yeniden oluşturulurken UI'yi 2–4.5 sn bloke ederdi. Bu yeniden
oluşturmalar artık önceki liste çizilirken bir işçi iş parçacığında çalışır, böylece en uzun
duraklama 26–64 ms'dir (M serisi Mac). Düzenlemeler hâlâ UI iş parçacığında yeniden oluşturulur (o
çizimde yaklaşık 1.8 sn): varlık başına önbellekleme sonraki adımdır.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-13 | küçük | Yakınlaştırma ve kaydırma arka planda yeniden oluşturulur: taranmış bir DWG'de 2–4.5 sn duraklamalar 26–64 ms'ye düşer |
| 2026-10-10 | büyük | İlk donanım paritesi belgesi: platform başına GPU arka uçları, girdi aygıtları, ekranlar, çiziciler |
