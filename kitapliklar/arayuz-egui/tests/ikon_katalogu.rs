//! P4 ikon kataloğu: 141 CAD ikonunun çizilebilirlik testi ve görsel denetim sayfası.
#![allow(clippy::disallowed_methods)]

use egui::{Color32, RichText, Sense, vec2};
use egui_kittest::Harness;
use spcad_ui_egui::simgeler::{self, Icon};
use spcad_ui_egui::tema;

#[test]
fn her_cad_ikonu_cizilebiliyor() {
    let mut h = Harness::new_ui(|ui| {
        ui.horizontal_wrapped(|ui| {
            for ikon in Icon::ALL {
                let (rect, _) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::hover());
                simgeler::paint(ui.painter(), rect.shrink(1.0), *ikon, false);
            }
        });
    });
    h.run();
    assert_eq!(Icon::ALL.len(), 141, "ikon sayısı değişti: listeyi güncelle");
}

#[test]
#[ignore = "P4 görsel denetim sayfası; PNG üretir, elle çalıştırılır"]
fn katalog_sayfasi() {
    let mut h = Harness::builder().with_size(vec2(1400.0, 950.0)).wgpu().build_ui(|ui| {
        ui.label(RichText::new("SPCAD CAD ikon kataloğu — 24×24 grid, 1,5 px hedef stroke").size(13.0).color(Color32::from_rgb(0xE6, 0xEB, 0xF2)));
        ui.add_space(8.0);
        egui::Grid::new("ikon_katalogu").num_columns(10).spacing(vec2(12.0, 10.0)).show(ui, |ui| {
            for (i, ikon) in Icon::ALL.iter().enumerate() {
                ui.vertical(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::hover());
                    simgeler::paint(ui.painter(), rect, *ikon, false);
                    ui.label(RichText::new(format!("{ikon:?}")).size(8.0).color(Color32::from_rgb(0x87, 0x94, 0xA2)));
                });
                if (i + 1) % 10 == 0 {
                    ui.end_row();
                }
            }
        });
    });
    tema::apply(&h.ctx, egui::Theme::Dark, false);
    h.run();
    let img = h.render().expect("render");
    let dizin = concat!(env!("CARGO_MANIFEST_DIR"), "/../../belgeler/gorseller/wp4");
    std::fs::create_dir_all(dizin).expect("dizin");
    img.save(format!("{dizin}/ikon-katalogu.png")).expect("png");
}
