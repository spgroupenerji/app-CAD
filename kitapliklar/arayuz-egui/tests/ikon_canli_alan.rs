//! P4: 141 CAD ikonunun 24×24 hücredeki mürekkep sınırlarını ölçer.
#![allow(clippy::disallowed_methods)]

use egui::{Rect, pos2, vec2};
use egui_kittest::Harness;
use spcad_ui_egui::simgeler::{self, Icon};
use spcad_ui_egui::tema;

const HUcre: f32 = 32.0;
const KOLON: usize = 12;

#[test]
fn ikon_canli_alan_denetimi() {
    let satir = Icon::ALL.len().div_ceil(KOLON);
    let mut h = Harness::builder()
        .with_size(vec2(KOLON as f32 * HUcre, satir as f32 * HUcre))
        .wgpu()
        .build_ui(|ui| {
            let p = ui.painter();
            for (i, ikon) in Icon::ALL.iter().enumerate() {
                let x = (i % KOLON) as f32 * HUcre + 4.0;
                let y = (i / KOLON) as f32 * HUcre + 4.0;
                simgeler::paint(p, Rect::from_min_size(pos2(x, y), vec2(24.0, 24.0)), *ikon, false);
            }
        });
    tema::install_fonts(&h.ctx, "en");
    tema::apply(&h.ctx, egui::Theme::Dark, false);
    h.run();
    let img = h.render().expect("render");
    let mut kucuk = Vec::new();
    let mut bos = Vec::new();
    let mut en_kucuk = f32::MAX;
    for (i, ikon) in Icon::ALL.iter().enumerate() {
        let cx = (i % KOLON) as u32 * HUcre as u32 + 4;
        let cy = (i / KOLON) as u32 * HUcre as u32 + 4;
        let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0u32, 0u32);
        for y in cy..cy + 24 {
            for x in cx..cx + 24 {
                if img.get_pixel(x, y)[3] > 8 {
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x);
                    y1 = y1.max(y);
                }
            }
        }
        if x0 == u32::MAX {
            bos.push(format!("{ikon:?}"));
            continue;
        }
        let genislik = (x1 - x0 + 1) as f32;
        let yukseklik = (y1 - y0 + 1) as f32;
        let d = genislik.max(yukseklik);
        en_kucuk = en_kucuk.min(d);
        if d < 14.0 {
            kucuk.push(format!("{ikon:?}={d:.0}px"));
        }
    }
    println!("ikon={} satir={} en_kucuk_canli_alan={en_kucuk:.0}px", Icon::ALL.len(), satir);
    if !kucuk.is_empty() {
        println!("kucuk ({}): {}", kucuk.len(), kucuk.join(", "));
    }
    assert!(bos.is_empty(), "hiç mürekkep yok: {bos:?}");
    assert!(kucuk.len() <= 6, "14 px altı ikon çok fazla: {kucuk:?}");
}
