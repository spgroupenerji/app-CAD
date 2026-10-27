//! P5: tasarım tokenlarının gerçekten piksele ulaştığını doğrular (platformdan bağımsız).
#![allow(clippy::disallowed_methods)]

use egui::{Color32, Sense, pos2, vec2};
use egui_kittest::Harness;
use spcad_ui_egui::tema::{self, Tokens};

fn parlaklik(p: &image::Rgba<u8>) -> u32 {
    u32::from(p[0]) + u32::from(p[1]) + u32::from(p[2])
}

#[test]
fn token_renkleri_piksele_ulasiyor() {
    let mut h = Harness::builder()
        .with_size(vec2(320.0, 120.0))
        .wgpu()
        .build_ui(|ui| {
            let t = Tokens::get();
            let p = ui.painter();
            for (i, renk) in [t.canvas, t.panel, t.control, t.accent, t.field].into_iter().enumerate() {
                let r = egui::Rect::from_min_size(pos2(10.0 + i as f32 * 60.0, 20.0), vec2(50.0, 80.0));
                p.rect_filled(r, 0.0, renk);
            }
        });
    tema::apply(&h.ctx, egui::Theme::Dark, false);
    h.run();
    let img = h.render().expect("render");
    let ornek = |i: u32| img.get_pixel(35 + i * 60, 60);
    let canvas = ornek(0);
    let panel = ornek(1);
    let accent = ornek(3);
    let field = ornek(4);
    assert_ne!(canvas, panel, "tuval ve panel aynı pikseli üretmemeli");
    assert!(parlaklik(panel) > parlaklik(canvas), "panel tuvalden açık olmalı");
    assert!(parlaklik(accent) > parlaklik(panel), "aksan panelden dikkat çekici olmalı");
    assert_ne!(field, panel, "giriş alanı panelden ayrışmalı");
    // Boş alan arka planı çizilmemiş olmalı (şeffaf/siyah).
    let bos = img.get_pixel(310, 110);
    assert!(parlaklik(bos) < parlaklik(panel), "çizilmemiş alan panelden koyu olmalı");
}
