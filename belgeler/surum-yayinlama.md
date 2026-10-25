# SPCAD sürümünü yayınlama

Kanonik tarif craftrules'un
[`release/playbook.md`](https://github.com/storytold/craftrules/blob/main/release/playbook.md)
belgesidir; bu dosya yalnızca SPCAD'e özgü kısımları kapsar.

- **Sürüm:** Kök `Cargo.toml` içindeki `[workspace.package] version` tek kaynaktır (semver
  çekirdeği, `MAJOR.MINOR.PATCH`). Her ön yüz `MAJOR.MINOR.PATCH_vYYYYMMDDHHMM` gösterir:
  `kitapliklar/motor/build.rs` UTC derleme damgasını gömer; damga `SPCAD_BUILD_STAMP=YYYYMMDDHHMM`
  ile sabitlenir. `cargo xtask version` çekirdeği yazdırır; `cargo xtask version set X.Y.Z[-pre]`
  çekirdeği değiştirir. Uygulama görünen sürümü Hakkında penceresinde, `spcad --version`,
  `spcad-cli --version`, web hazır iletisi ve MCP `serverInfo` alanında gösterir; sürüm
  derlemeleri ayrıca commit'i gömer (`SPCAD_BUILD_SHA`).
- **İndirilebilir beta:** `cargo xtask release` damgayı (`vYYYYMMDDHHMM`, Türkiye yerel saati) bir kez üretir; aynı damga `release/sp-group-cad_beta-<damga>.exe` adına ve `SPCAD_BUILD_STAMP` üzerinden Hakkında penceresine gider. Kök `release/` yalnız son derlemeyi tutar (eskiler silinir) ve indirilebilir olması için depoya işlenir.
- **Sürüm çıkarma:** `main` üzerindeki sürümü artırın, sonra `main` dalını `release` dalına
  gönderin (`git push origin main:release`). `release` dalına yapılan her push
  `.github/workflows/release.yml` iş akışını çalıştırır ve `SPCAD v<version>` taslak GitHub
  sürümünü oluşturur veya günceller. `version: X.Y.Z-rc.N` ile yapılan bir `workflow_dispatch`
  çalışması test derlemesi üretir. Başka bir dalda yapılan dispatch deneme çalışmasıdır: imzasız
  işler (sürüm, Linux, Linux riscv64, Flatpak, FreeBSD, web) derlenir, `release` ortamı imzalama
  işlerini (macOS, Windows) reddeder ve hiçbir sürüm taslağı oluşturulmaz.
- **Yapıtlar:** macOS evrensel `.dmg` (Developer ID ile imzalı, noter onaylı, staplenmiş) ve CLI
  `.zip`; Windows x64, x86 ve arm64 `.msi` + taşınabilir `.zip` (Azure Trusted Signing); Linux
  x86_64 ve aarch64 `.AppImage` (AppImageUpdate delta güncellemeleri için `.AppImage.zsync`),
  `.flatpak` paketi (tarball'ı yeniden paketler, `paketleme/linux/flatpak-bundle.sh`), `.deb`,
  `.rpm`, `.tar.gz` (+ `paketleme/linux/flatpak/` içindeki kaynaktan Flathub bildirimi); Linux
  riscv64 `.tar.gz` (Ubuntu 24.04 üzerinde ports.ubuntu.com riscv64 kitaplıklarına karşı çapraz
  derlenir, CLI QEMU altında duman testinden geçirilir); FreeBSD x86_64 `.tar.gz` (FreeBSD
  VM'inde derlenir); web `.zip` (wasm + `index.html`, bkz. `paketleme/web/README.md`);
  `SHA256SUMS.txt`.
- **Gizli anahtarlar:** On iki imzalama gizli anahtarı `release` ortamında bulunur (yalnızca
  `release` dalından dağıtılabilir; bu dala yalnızca kuruluş yöneticileri ve sürüm yöneticileri
  push yapabilir). Hepsi isteğe bağlıdır: eksik olan biri uyarıyla imzasız yapıt üretir.
- **Windows yükleyicisi:** `paketleme/windows/spcad.wxs` (WiX v5 + `WixToolset.UI.wixext`).
  Sihirbazında bir Kısayollar sayfası vardır: Başlat Menüsü kısayolu her zaman oluşturulur,
  masaüstü kısayolu isteğe bağlıdır (sessiz kurulumlarda `DESKTOPSHORTCUT=1`, yükseltmede
  `DESKTOPSHORTCUT=0` kısayolu kaldırır; aksi hâlde yükseltmeler önceki seçimi korur).
- **Dosya türleri:** `.dxf` ve `.dwg` (macOS belge türleri, Windows "Birlikte Aç", Linux MIME
  türleri).
- **Yerel paketleme testi:** `paketleme/macos/package.sh` (ad-hoc imzalı DMG),
  `paketleme/linux/package.sh`, `paketleme/web/package.sh`.
