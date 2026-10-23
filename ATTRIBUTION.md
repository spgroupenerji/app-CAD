# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example drawings, presets) is listed
here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if
an asset file is missing from this table.

**Policy (mandatory):** This distribution contains **no Autodesk, Adobe or Avid iconography, images,
artwork, fonts, hatch patterns, linetypes, templates or presets.** Every asset is original work by
SPCAD contributors or third-party material under an open licence (OSI open source, public domain
/ CC0, or Creative Commons that allows redistribution). Screenshots of Autodesk software are never
committed. Font files are not added here: shared fonts live in
[storytold/craft-fonts](https://github.com/storytold/craft-fonts), an optional build input. The one
exception is the SP-GROUP brand: `varliklar/uygulama-simgesi/sp-group-cad.png` and the icons derived from it are
SP-GROUP brand assets, not open source, distributed with this repository.

Generated-in-code assets are original and have no file to list:
- the UI icon set (`kitapliklar/arayuz-egui/src/simgeler.rs`);
- the "SPCAD Stroke" single-stroke drafting font (`kitapliklar/yazitipi/src/tek_cizgi.rs`);
- the standard linetype and hatch-pattern libraries (`kitapliklar/belge/src/kutuphane.rs`; industry-common
  names, our own dash/spacing values);
- the colour index palette, generated from its structure (`kitapliklar/renk/src/lib.rs`);
- the sample drawings (`kitapliklar/motor/src/ornek.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP | original SP-GROUP CAD brand asset supplied by the project owner | SP-GROUP brand asset (not open source); distributed with this repository | Master logo; every other app icon derives from it |
| `varliklar/uygulama-simgesi/hicolor/128x128/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/16x16/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/24x24/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/256x256/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/32x32/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/48x48/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/512x512/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/hicolor/64x64/apps/com.spgroup.spcad.png` | SP-GROUP | generated from `varliklar/uygulama-simgesi/sp-group-cad.png` | SP-GROUP brand asset | Linux hicolor icon |
| `varliklar/uygulama-simgesi/spcad.icns` | SP-GROUP | generated (`paketleme/ikonlar.py`) | SP-GROUP brand asset | macOS icon |
| `varliklar/uygulama-simgesi/spcad.ico` | SP-GROUP | generated (`paketleme/ikonlar.py`) | SP-GROUP brand asset | Windows icon |
| `varliklar/uygulama-simgesi/spcad-64.png` | SP-GROUP | generated (`paketleme/ikonlar.py`) | SP-GROUP brand asset | |
| `varliklar/uygulama-simgesi/spcad-256.png` | SP-GROUP | generated (`paketleme/ikonlar.py`) | SP-GROUP brand asset | |
| `varliklar/uygulama-simgesi/spcad-1024.png` | SP-GROUP | generated (`paketleme/ikonlar.py`) | SP-GROUP brand asset | |
| `varliklar/uygulama-simgesi/spcad-macos-512.png` | SP-GROUP | generated (`paketleme/ikonlar.py`, macOS margin variant) | SP-GROUP brand asset | |
| `paketleme/macos/dmg/background.svg` | @XusBadia | original work for this repository (2026-10-08), "SP-GROUP CAD macOS DMG window background"; derives from the SP-GROUP app icon (`varliklar/uygulama-simgesi/`, referenced via `<image>`, not copied); wordmark removed and icon refitted by SP-GROUP (2026-10-22); text is outlined Inter / JetBrains Mono glyph paths (OFL fonts the project already uses); no third-party material modified | MIT OR Apache-2.0 (same as the code: `LICENSE-MIT`, `LICENSE-APACHE`) | Finder window background of the macOS DMG; `paketleme/macos/dmg/generate.py` renders it to `background.tiff` (see `paketleme/macos/dmg/README.md`) |
| `paketleme/macos/dmg/background.tiff` | @XusBadia | Rendered from `paketleme/macos/dmg/background.svg` by `paketleme/macos/dmg/generate.py` | MIT OR Apache-2.0 (same as the code: `LICENSE-MIT`, `LICENSE-APACHE`) | 1x + 2x HiDPI TIFF for the macOS DMG window; includes the app icon (see its row) and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `paketleme/macos/dmg/dmg-layout.DS_Store` | @XusBadia | Generated by `paketleme/macos/dmg/generate.py` | MIT OR Apache-2.0 (same as the code: `LICENSE-MIT`, `LICENSE-APACHE`) | Finder view settings for the DMG window; no machine-specific data |
| `ornekler/apartment.dxf` | CADCraft contributors | sample drawing made with spcad-cli commands | MIT OR Apache-2.0 | Original |
| `belgeler/gorseller/ui-apartment.png` | SP-GROUP | screenshot of SP-GROUP CAD itself (ornekler/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `belgeler/gorseller/ui-bracket.png` | SP-GROUP | screenshot of SP-GROUP CAD itself (sample drawing generated in code) | MIT OR Apache-2.0 | Original; no Autodesk UI |

## Interface translations

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `kitapliklar/arayuz-egui/src/i18n/uk.tsv` | @dmatviichuk | Original Ukrainian translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/de.tsv` | the CADCraft contributors | Original German translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/es.tsv` | @PedroGuille | Original Spanish translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/pt_br.tsv` | @WladmirJunior | Original Brazilian Portuguese translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/cs.tsv` | @Losminek (AI-assisted) | Original Czech translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/ru.tsv` | @pechnikov2026-star | Original Russian translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/fr.tsv` | @pechnikov2026-star | Original French translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/zh.tsv` | @mayloglog | Original Simplified Chinese translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/sv.tsv` | Erik Holmbom (@holmbom) with Claude Code (AI-assisted) | Original Swedish translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; terminology and the running app reviewed by a native speaker |
| `kitapliklar/arayuz-egui/src/i18n/it.tsv` | Emanuele Bottelli (AI-assisted) | Original Italian translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/ko.tsv` | @huk082 (AI-assisted) | Original Korean translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; no font assets added |
| `kitapliklar/arayuz-egui/src/i18n/ar.tsv` | @hasan-sdaif | Original Arabic translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/hu.tsv` | @Skaywalker | Original Hungarian translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `kitapliklar/arayuz-egui/src/i18n/tr.tsv` | @ogzhnklvz (AI-assisted) | Original Turkish translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; written from the English strings, no other catalog used as a source; the running app reviewed by a native speaker |
| `kitapliklar/arayuz-egui/src/i18n/el.tsv` | @Camthalion77 with ChatGPT (AI-assisted) | Original Greek translations of SPCAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; independent native proofreading pending |
