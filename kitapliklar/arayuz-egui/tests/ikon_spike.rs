//! P0 ikon spike'ı: iconflow/Lucide fontu ile mevcut \`simgeler.rs\` çizimini aynı
//! egui bağlamında, 1×/1,25×/1,5×/2× ölçekte yan yana çizer ve PNG üretir.
//!
//! Çalıştırma: cargo test -p spcad-ui-egui --test ikon_spike -- --nocapture

#![allow(clippy::disallowed_methods)]

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, RichText, Sense, vec2};
use egui_kittest::Harness;
use iconflow::{Pack, Size, Style, fonts, try_icon};
use spcad_ui_egui::simgeler::{self, Icon};
use spcad_ui_egui::tema;

/// Lucide adları: genel arayüz + CAD komutlarının karşılığı olan temsilî set.
const IKONLAR: &[&str] = &[
    "house", "search", "settings", "layers", "move", "rotate-cw", "copy", "trash-2", "zoom-in",
    "panel-left", "chevron-down", "x", "check", "plus", "minus", "file-plus", "folder-open",
    "ruler", "pencil", "square", "circle", "sliders-horizontal", "table", "command",
];

const MEVCUT: &[(Icon, &str)] = &[
    (Icon::Line, "Line"),
    (Icon::Circle, "Circle"),
    (Icon::Arc, "Arc"),
    (Icon::Move, "Move"),
    (Icon::Copy, "Copy"),
    (Icon::Rotate, "Rotate"),
    (Icon::Trim, "Trim"),
    (Icon::Hatch, "Hatch"),
    (Icon::Layers, "Layers"),
    (Icon::Properties, "Props"),
];

fn fontlari_kur(ctx: &egui::Context) {
    let mut tanimlar = FontDefinitions::default();
    for font in fonts() {
        let ad = font.family.to_string();
        tanimlar.font_data.insert(ad.clone(), std::sync::Arc::new(FontData::from_static(font.bytes)));
        // Glyph'ler özel kullanım alanında; Latin fontlardan sonra geri düşüş olarak eklenir.
        for aile in [FontFamily::Proportional, FontFamily::Monospace] {
            if let Some(liste) = tanimlar.families.get_mut(&aile) {
                liste.push(ad.clone());
            }
        }
    }
    ctx.set_fonts(tanimlar);
}

/// Lucide ikonunun glyph'i; yoksa None.
fn lucide(ad: &str) -> Option<char> {
    let ikon = try_icon(Pack::Lucide, ad, Style::Regular, Size::Regular).ok()?;
    char::from_u32(ikon.codepoint)
}

fn sayfa(ui: &mut egui::Ui) {
    ui.label(RichText::new("iconflow / Lucide — 14 · 16 · 18 · 20 px").size(12.0).color(Color32::from_rgb(0xAE, 0xB9, 0xC6)));
    for boyut in [14.0_f32, 16.0, 18.0, 20.0] {
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(vec2(34.0, boyut + 6.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(RichText::new(format!("{boyut:.0}")).size(10.0).color(Color32::from_rgb(0x87, 0x94, 0xA2)));
            });
            for ad in IKONLAR {
                let Some(ch) = lucide(ad) else { continue };
                let fid = FontId::new(boyut, FontFamily::Proportional);
                ui.label(RichText::new(ch.to_string()).font(fid).color(Color32::from_rgb(0xE6, 0xEB, 0xF2)));
                ui.add_space(6.0);
            }
        });
    }
    ui.add_space(10.0);
    ui.label(RichText::new("mevcut simgeler.rs — 20 px").size(12.0).color(Color32::from_rgb(0xAE, 0xB9, 0xC6)));
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(vec2(34.0, 26.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.label(RichText::new("20").size(10.0).color(Color32::from_rgb(0x87, 0x94, 0xA2)));
        });
        for (ikon, ad) in MEVCUT {
            let (rect, _) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::hover());
            simgeler::paint(ui.painter(), rect.shrink(2.0), *ikon, false);
            ui.add_space(2.0);
            ui.label(RichText::new(*ad).size(9.0).color(Color32::from_rgb(0x87, 0x94, 0xA2)));
            ui.add_space(6.0);
        }
    });
}

#[test]
#[ignore = "P0 ölçüm sayfası; PNG üretir, elle çalıştırılır"]
fn ikon_spike_sayfasi() {
    let mut h = Harness::builder()
        .with_size(vec2(1500.0, 420.0))
        .wgpu()
        .build_ui(|ui| {
            egui::Frame::NONE.fill(Color32::from_rgb(0x0C, 0x0F, 0x13)).inner_margin(16.0).show(ui, sayfa);
        });
    fontlari_kur(&h.ctx);
    tema::apply(&h.ctx, egui::Theme::Dark, false);
    h.run();
    let dizin = concat!(env!("CARGO_MANIFEST_DIR"), "/../../belgeler/gorseller/wp2");
    std::fs::create_dir_all(dizin).expect("çıktı dizini");
    for (etiket, ppp) in [("1x", 1.0_f32), ("1_25x", 1.25), ("1_5x", 1.5), ("2x", 2.0)] {
        h.set_pixels_per_point(ppp);
        h.run();
        let goruntu = h.render().expect("render");
        let yol = format!("{dizin}/ikon-spike-{etiket}.png");
        goruntu.save(&yol).expect("PNG yaz");
        println!("yazıldı: {yol}");
    }
}