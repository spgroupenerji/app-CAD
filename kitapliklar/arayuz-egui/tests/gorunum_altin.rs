//! P5 golden altyapısı: sabit durumdaki kromun referans PNG'lerini üretir.
//! Çalıştırma: cargo test -p spcad-ui-egui --test gorunum_altin -- --ignored --nocapture
#![allow(clippy::disallowed_methods)]

use egui::vec2;
use egui_kittest::Harness;
use spcad_engine::Session;
use spcad_ui_egui::{CadApp, Services};

const DIZIN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../belgeler/gorseller/altin");

fn sahne(dialog: Option<&'static str>, gecmis: usize) -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.in_window_menu = false;
    app.integrated_titlebar = true;
    app.ui.show_palettes = true;
    app.ui.show_shelf = true;
    app.ui.show_command_line = true;
    app.ui.show_status_bar = true;
    app.ui.show_viewcube = true;
    app.ui.show_file_tabs = true;
    app.ui.history_lines = gecmis;
    let mut h = Harness::builder()
        .with_size(vec2(1600.0, 1000.0))
        .wgpu()
        .build_ui_state(
            move |ui, app: &mut CadApp| {
                if let Some(d) = dialog {
                    app.ui.dialog = Some(d.into());
                }
                app.logic(&ui.ctx().clone());
                app.ui(ui);
            },
            app,
        );
    h.run_steps(6);
    h
}

fn kaydet(h: &mut Harness<'static, CadApp>, ad: &str) {
    std::fs::create_dir_all(DIZIN).expect("dizin");
    let img = h.render().expect("render");
    let yol = format!("{DIZIN}/{ad}.png");
    img.save(&yol).expect("png");
    println!("yazıldı: {yol}");
}

#[test]
#[ignore = "referans PNG üretir; elle çalıştırılır"]
fn altin_goruntuleri() {
    let mut h = sahne(None, 3);
    kaydet(&mut h, "cizim");
    let mut h = sahne(Some("options"), 6);
    kaydet(&mut h, "diyalog");
    let mut h = sahne(Some("layers"), 6);
    kaydet(&mut h, "katmanlar");
}
