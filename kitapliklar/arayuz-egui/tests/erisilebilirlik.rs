//! P5 erişilebilirlik duman testi: temel krom kontrolleri erişilebilir etiket taşır.
#![allow(clippy::disallowed_methods)]

use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use spcad_engine::Session;
use spcad_ui_egui::{CadApp, Services};

fn harness() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.in_window_menu = false;
    app.integrated_titlebar = true;
    let mut h = Harness::builder().with_size(vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h
}

#[test]
fn temel_krom_kontrolleri_etiketli() {
    let h = harness();
    for etiket in ["Draw", "Output", "Line", "Home"] {
        assert!(h.query_by_label(etiket).is_some(), "erişilebilirlik etiketi yok: {etiket}");
    }
}

#[test]
fn durum_cubugu_anahtarlari_etiketli() {
    let h = harness();
    for etiket in ["Grid (F7)", "Snap Mode (F9)", "Ortho Mode (F8)", "Polar Tracking (F10)", "Object Snap (F3)"] {
        assert!(h.query_by_label(etiket).is_some(), "erişilebilirlik etiketi yok: {etiket}");
    }
}
