//! The Options dialog. It reads the engine's settings and the interface state, and changes them
//! only through the OPTIONS JSON form (`app.run("options", …)`), so the dialog, scripts and the
//! control channel share one path. Changes apply at once.
//!
//! The open tab is part of the dialog name (`"options:selection"`), so `ui.set` can show any tab.

#![allow(clippy::disallowed_methods)]

use egui::{Color32, Rect, RichText, Sense, Stroke, pos2, vec2};
use serde_json::{Map, Value, json};

use crate::CadApp;
use crate::animasyon::AnimationPref;
use crate::tema::{ThemePref, Tokens};

/// The tabs: (id in the dialog name, label).
pub(super) const TABS: [(&str, &str); 6] = [
    ("display", "Display"),
    ("files", "Open and Save"),
    ("drafting", "Drafting"),
    ("selection", "Selection"),
    ("preferences", "User Preferences"),
    ("profiles", "Profiles"),
];

const WIDTH: f32 = 760.0;
const MIN_TAB_HEIGHT: f32 = 440.0;
const TAB_H: f32 = 36.0;

/// Show the Options dialog (`name` is `"options"` or `"options:<tab>"`).
pub fn dialog(app: &mut CadApp, ctx: &egui::Context, name: &str, open: &mut bool) {
    let tab = name.strip_prefix("options:").and_then(|t| TABS.iter().find(|(id, _)| *id == t)).map_or("display", |(id, _)| *id);
    let mut changes = Map::new();
    let mut next_tab = None;
    let mut start = None;
    let mut close = false;
    let center = ctx.content_rect().center();
    let title = app.session.state().map(|s| s.title.clone()).unwrap_or_default();
    egui::Window::new(crate::tl!("Options"))
        .id(egui::Id::new("cc_options"))
        .open(open)
        .resizable(false)
        .collapsible(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(center)
        .show(ctx, |ui| {
            ui.set_width(WIDTH);
            ui.horizontal(|ui| {
                ui.label(crate::tl!("Current Profile:"));
                ui.label(RichText::new(crate::tl!("Default")).color(Tokens::get().text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(if title.is_empty() { crate::tl!("No drawing").to_string() } else { title.clone() });
                    ui.label(crate::tl!("Current Drawing:"));
                });
            });
            ui.add_space(4.0);
            if let Some(t) = sekme_seridi(ui, tab) {
                next_tab = Some(t);
            }
            egui::Frame::NONE.fill(Tokens::get().chrome_dark).inner_margin(egui::Margin::symmetric(18, 14)).show(ui, |ui| {
                ui.set_width(WIDTH - 4.0);
                let top = ui.cursor().top();
                match tab {
                    "files" => files(app, ui, &mut changes),
                    "drafting" => drafting(app, ui, &mut changes, &mut start),
                    "selection" => selection(app, ui, &mut changes),
                    "preferences" => preferences(app, ui, &mut changes),
                    "profiles" => profiles(app, ui, &mut changes),
                    _ => display(app, ui, &mut changes),
                }
                // The same height on every tab, so switching tabs doesn't move the buttons.
                ui.add_space((MIN_TAB_HEIGHT - (ui.cursor().top() - top)).max(8.0));
            });
            ui.add_space(6.0);
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let total = 2.0 * 96.0 + 8.0;
                    ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
                    if ui.add_sized(vec2(96.0, 24.0), egui::Button::new(crate::tl!("OK"))).clicked() {
                        close = true;
                    }
                    if ui.add_sized(vec2(96.0, 24.0), egui::Button::new(crate::tl!("Help"))).clicked() {
                        start = Some("ui.dialog.about");
                    }
                });
            });
            ui.add_space(2.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(crate::tl!("Changes apply at once and are kept for next time.")).small().color(Tokens::get().text_faint));
            });
        });
    if close {
        *open = false;
    }
    if let Some(t) = next_tab {
        app.ui.dialog = Some(format!("options:{t}"));
    }
    if !changes.is_empty() {
        let _ = app.run("options", Value::Object(changes));
    }
    if let Some(id) = start {
        app.start(id);
    }
}

/// A row of tabs across the dialog, the open one joined to the content below.
fn sekme_seridi(ui: &mut egui::Ui, open: &str) -> Option<&'static str> {
    let t = Tokens::get();
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), TAB_H), Sense::hover());
    ui.painter().rect_filled(Rect::from_min_max(pos2(r.left(), r.bottom() - 2.0), r.max), 0.0, t.panel);
    let mut x = r.left();
    let mut clicked = None;
    for (id, label) in TABS {
        let g = ui.painter().layout_no_wrap(crate::i18n::t(label).to_string(), egui::FontId::proportional(12.5), t.text);
        let w = g.size().x + 26.0;
        let tr = Rect::from_min_size(pos2(x, r.top() + 4.0), vec2(w, r.height() - 4.0));
        let resp = ui.interact(tr, ui.id().with(("opt_tab", id)), Sense::click());
        let on = id == open;
        let fill = if on {
            t.panel
        } else if resp.hovered() {
            t.control_hover.gamma_multiply(0.55)
        } else {
            t.chrome
        };
        ui.painter().rect_filled(tr, egui::CornerRadius { nw: 4, ne: 4, sw: 0, se: 0 }, fill);
        ui.painter().line_segment([tr.left_top(), tr.right_top()], Stroke::new(1.0, t.border));
        ui.painter().line_segment([tr.left_top(), tr.left_bottom()], Stroke::new(1.0, t.border));
        ui.painter().line_segment([tr.right_top(), tr.right_bottom()], Stroke::new(1.0, t.border));
        if on {
            ui.painter().rect_filled(Rect::from_min_max(tr.left_top(), pos2(tr.right(), tr.top() + 2.0)), 1.0, t.accent);
        } else {
            ui.painter().line_segment([tr.left_bottom(), tr.right_bottom()], Stroke::new(1.0, t.border));
        }
        let color = if on { t.text } else { t.text_dim };
        ui.painter().galley(pos2(tr.center().x - g.size().x / 2.0, tr.center().y - g.size().y / 2.0), g, color);
        crate::simgeler::describe_control(ui, &resp, label, Some(on), true);
        if resp.clicked() && !on {
            clicked = Some(id);
        }
        x += w;
    }
    clicked
}

/// A framed group with its title written over the top border, as in the reference dialog.
fn kutu(ui: &mut egui::Ui, baslik: &str, add: impl FnOnce(&mut egui::Ui)) {
    let t = Tokens::get();
    ui.add_space(9.0);
    let out = egui::Frame::NONE
        .fill(t.panel)
        .stroke(Stroke::new(1.0, t.border))
        .inner_margin(egui::Margin { left: 10, right: 10, top: 12, bottom: 10 })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui);
        });
    let r = out.response.rect;
    let g = ui.painter().layout_no_wrap(baslik.to_string(), egui::FontId::proportional(12.5), t.text);
    let ty = r.top() - g.size().y / 2.0 - 1.0;
    ui.painter().rect_filled(Rect::from_min_size(pos2(r.left() + 8.0, ty), g.size() + vec2(8.0, 0.0)), 0.0, t.panel);
    ui.painter().galley(pos2(r.left() + 12.0, ty), g, t.text);
}

fn grid(ui: &mut egui::Ui, id: &str, add: impl FnOnce(&mut egui::Ui)) {
    egui::Grid::new(id).num_columns(2).spacing(vec2(14.0, 8.0)).show(ui, add);
}

fn hint(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).small().color(Tokens::get().text_dim));
}

fn theme_label(t: ThemePref) -> &'static str {
    match t {
        ThemePref::System => crate::tl!("Use System Setting"),
        ThemePref::Light => crate::tl!("Light"),
        ThemePref::Dark => crate::tl!("Dark"),
    }
}

fn animations_label(a: AnimationPref) -> &'static str {
    match a {
        AnimationPref::System => crate::tl!("Use System Setting"),
        AnimationPref::On => crate::tl!("On"),
        AnimationPref::Off => crate::tl!("Off"),
    }
}

/// A colour button, its hex value and a Default button; a change goes into `changes` under `key`.
fn color_row(ui: &mut egui::Ui, current: Option<Color32>, default: Color32, key: &str, changes: &mut Map<String, Value>) {
    ui.horizontal(|ui| {
        let shown = current.unwrap_or(default);
        let mut rgb = [shown.r(), shown.g(), shown.b()];
        if ui.color_edit_button_srgb(&mut rgb).changed() {
            changes.insert(key.into(), json!(super::hex(Color32::from_rgb(rgb[0], rgb[1], rgb[2]))));
        }
        ui.monospace(super::hex(shown));
        if ui.add_enabled(current.is_some(), egui::Button::new(crate::tl!("Default"))).clicked() {
            changes.insert(key.into(), Value::Null);
        }
    });
}

/// A square of `side` points in `color` on the drawing-area background (marker and pickbox
/// previews).
fn preview_square(ui: &mut egui::Ui, side: f32, color: Color32) {
    let t = Tokens::get();
    let (rect, _) = ui.allocate_exact_size(vec2(36.0, 36.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, 3.0, t.canvas);
    let side = side.clamp(1.0, 32.0);
    p.rect_stroke(egui::Rect::from_center_size(rect.center(), vec2(side, side)), 0.0, Stroke::new(2.0, color), egui::StrokeKind::Middle);
}

fn display(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>) {
    let mut system_cursor = app.ui.system_cursor;
    ui.columns(2, |cols| {
        let [left, right] = cols else { return };
        kutu(left, crate::tl!("Window Elements"), |ui| {
            grid(ui, "options_display_window", |ui| {
                ui.label(crate::tl!("Interface theme"));
                let mut theme = app.ui.theme;
                egui::ComboBox::from_id_salt("options_theme").selected_text(theme_label(theme)).show_ui(ui, |ui| {
                    for t in ThemePref::ALL {
                        ui.selectable_value(&mut theme, t, theme_label(t));
                    }
                });
                if theme != app.ui.theme {
                    changes.insert("theme".into(), json!(theme.as_str()));
                }
                ui.end_row();

                ui.label(crate::tl!("Animation effects"));
                let mut animations = app.ui.animations;
                egui::ComboBox::from_id_salt("options_animations").selected_text(animations_label(animations)).show_ui(ui, |ui| {
                    for a in AnimationPref::ALL {
                        ui.selectable_value(&mut animations, a, animations_label(a));
                    }
                });
                if animations != app.ui.animations {
                    changes.insert("animations".into(), json!(animations.as_str()));
                }
                ui.end_row();

                ui.label(crate::tl!("Interface scale"));
                let scale = app.options.interface_scale;
                let percent = |s: f32| format!("{} %", (s * 100.0).round());
                egui::ComboBox::from_id_salt("options_interface_scale").selected_text(percent(scale)).show_ui(ui, |ui| {
                    for n in [75u16, 100, 125, 150, 175, 200, 250, 300] {
                        let s = f32::from(n) / 100.0;
                        if ui.selectable_label((scale - s).abs() < 1e-4, format!("{n} %")).clicked() && (scale - s).abs() >= 1e-4 {
                            changes.insert("interfaceScale".into(), json!(s));
                        }
                    }
                });
                ui.end_row();
            });
            if left_checkbox(ui, &mut system_cursor, crate::tl!("Keep the system cursor visible over the drawing area")) {
                changes.insert("systemCursor".into(), json!(system_cursor));
            }
            hint(ui, crate::tl!("Screen magnifiers, screen readers and recorders follow the system cursor."));
        });
        kutu(right, crate::tl!("Drawing Area"), |ui| {
            grid(ui, "options_display_area", |ui| {
                ui.label(crate::tl!("Model background"));
                color_row(ui, app.options.model_background, Tokens::DARK.canvas, "modelBackground", changes);
                ui.end_row();

                ui.label(crate::tl!("Layout paper"));
                color_row(ui, app.options.paper_background, Tokens::DARK.paper, "paperBackground", changes);
                ui.end_row();

                ui.label(crate::tl!("Crosshair size"));
                let mut size = app.session.settings.cursorsize;
                if ui.add(egui::Slider::new(&mut size, 1.0..=100.0).integer().suffix(" %")).changed() {
                    changes.insert("CURSORSIZE".into(), json!(size));
                }
                ui.end_row();

                ui.label(crate::tl!("Command line history"));
                let mut lines = app.ui.history_lines;
                if ui.add(egui::DragValue::new(&mut lines).range(0..=crate::komut_satiri::MAX_LINES)).changed() {
                    changes.insert("historyLines".into(), json!(lines));
                }
                ui.end_row();
            });
        });
    });
}

/// A checkbox inside a group (kept apart so the borrowed value stays with its change handler).
fn left_checkbox(ui: &mut egui::Ui, value: &mut bool, text: &str) -> bool {
    ui.checkbox(value, text).changed()
}

fn files(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>) {
    ui.columns(2, |cols| {
        let [left, right] = cols else { return };
        kutu(left, crate::tl!("File Save"), |ui| {
            ui.label(crate::tl!("Save new drawings as"));
            ui.horizontal(|ui| {
                for f in crate::SaveFormat::ALL {
                    let label = match f {
                        crate::SaveFormat::Dxf => crate::tl!("DXF"),
                        crate::SaveFormat::Dwg => crate::tl!("DWG"),
                    };
                    if ui.radio(app.ui.save_format == f, label).clicked() && app.ui.save_format != f {
                        changes.insert("saveFormat".into(), json!(f.as_str()));
                    }
                }
            });
            ui.add_space(6.0);
            ui.label(crate::tl!("Default save version"));
            let current = crate::SaveVersion::parse(&app.ui.save_version).unwrap_or_default();
            let mut version = current;
            egui::ComboBox::from_id_salt("options_save_version").selected_text(version.year()).show_ui(ui, |ui| {
                for v in crate::SaveVersion::ALL {
                    ui.selectable_value(&mut version, v, v.year());
                }
            });
            if version != current {
                changes.insert("saveVersion".into(), json!(version.year()));
            }
            hint(ui, crate::tl!("A drawing opened from a file keeps that file's format."));
        });
        kutu(right, crate::tl!("File Safety Precautions"), |ui| {
            let mut minutes = spcad_engine::otokayit::savetime();
            let max = spcad_engine::otokayit::MAX_SAVETIME;
            ui.horizontal(|ui| {
                ui.label(crate::tl!("Automatic save every"));
                if ui.add(egui::DragValue::new(&mut minutes).range(0..=max).suffix(format!(" {}", crate::tl!("min")))).changed() {
                    changes.insert("SAVETIME".into(), json!(minutes));
                }
            });
            hint(ui, crate::tl!("0 minutes turns automatic save off."));
            ui.add_space(6.0);
            ui.label(crate::tl!("Automatic save file location"));
            // Edited as text; the folder changes when the field loses focus.
            let id = egui::Id::new("cc_options_savefilepath");
            let current = spcad_engine::otokayit::savefilepath();
            let mut path = ui.data(|d| d.get_temp::<String>(id)).unwrap_or_else(|| current.clone());
            let r = ui.add(egui::TextEdit::singleline(&mut path).desired_width(ui.available_width()));
            if r.changed() {
                ui.data_mut(|d| d.insert_temp(id, path.clone()));
            }
            if r.lost_focus() {
                ui.data_mut(|d| d.remove::<String>(id));
                if path.trim() != current {
                    changes.insert("SAVEFILEPATH".into(), json!(path.trim()));
                }
            }
            hint(ui, crate::tl!("Drawings with no file yet are saved in the default version."));
        });
    });
}

fn drafting(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>, start: &mut Option<&'static str>) {
    let t = Tokens::get();
    ui.columns(2, |cols| {
        let [left, right] = cols else { return };
        kutu(left, crate::tl!("Object Snap"), |ui| {
            grid(ui, "options_drafting_snap", |ui| {
                ui.label(crate::tl!("Object snap marker size"));
                ui.horizontal(|ui| {
                    let mut size = app.options.snap_marker_size;
                    if ui.add(egui::Slider::new(&mut size, super::SNAP_MARKER_SIZES).integer()).changed() {
                        changes.insert("snapMarkerSize".into(), json!(size));
                    }
                    preview_square(ui, size, t.snap);
                });
                ui.end_row();

                ui.label(crate::tl!("Object snap aperture"));
                ui.horizontal(|ui| {
                    let mut aperture = app.session.settings.aperture;
                    if ui.add(egui::Slider::new(&mut aperture, 1.0..=50.0).integer().suffix(" px")).changed() {
                        changes.insert("APERTURE".into(), json!(aperture));
                    }
                    preview_square(ui, aperture as f32, t.crosshair);
                });
                ui.end_row();
            });
            hint(ui, crate::tl!("The aperture is how near the cursor must come to a point for an object snap to find it."));
        });
        kutu(right, crate::tl!("Drafting Settings"), |ui| {
            ui.label(crate::tl!("Snap, grid, polar tracking and Dynamic Input:"));
            ui.add_space(4.0);
            if ui.button(crate::tl!("Drafting Settings...")).clicked() {
                *start = Some("dsettings");
            }
            ui.add_space(4.0);
            hint(ui, crate::tl!("Sets the spacing, angles and snap modes used while drawing."));
        });
    });
}

fn selection(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>) {
    let t = Tokens::get();
    let st = &app.session.settings;
    let mut effects = ui.ctx().data(|d| d.get_temp::<bool>(egui::Id::new(VISUAL_EFFECTS)).unwrap_or(false));
    ui.columns(2, |cols| {
        let [left, right] = cols else { return };
        kutu(left, crate::tl!("Pickbox size"), |ui| {
            ui.horizontal(|ui| {
                let mut pickbox = st.pickbox;
                if ui.add(egui::Slider::new(&mut pickbox, 0.0..=50.0).integer().suffix(" px")).changed() {
                    changes.insert("PICKBOX".into(), json!(pickbox));
                }
                preview_square(ui, (pickbox * 3.0) as f32, t.crosshair);
            });
        });
        kutu(left, crate::tl!("Selection modes"), |ui| {
            let mut pickfirst = st.pickfirst;
            if ui.checkbox(&mut pickfirst, crate::tl!("Select objects first, then choose a command")).changed() {
                changes.insert("PICKFIRST".into(), json!(i32::from(pickfirst)));
            }
            let mut shift_adds = !st.pickadd;
            if ui.checkbox(&mut shift_adds, crate::tl!("Hold Shift to add to the selection")).changed() {
                changes.insert("PICKADD".into(), json!(i32::from(!shift_adds)));
            }
            let mut grouping = st.pickstyle & 1 != 0;
            let mut two_clicks = !st.pickdrag;
            if ui
                .checkbox(&mut two_clicks, crate::tl!("Select using two clicks only"))
                .on_hover_text(crate::tl!("Define each corner of the selection window with a separate click."))
                .changed()
            {
                changes.insert("PICKDRAG".into(), json!(if two_clicks { 0 } else { 2 }));
            }
            if ui.checkbox(&mut grouping, crate::tl!("Object grouping")).changed() {
                changes.insert("PICKSTYLE".into(), json!((st.pickstyle & !1) | u8::from(grouping)));
            }
            let mut qp = st.qpmode;
            if ui.checkbox(&mut qp, crate::tl!("Show Quick Properties for selected objects")).changed() {
                changes.insert("QPMODE".into(), json!(i32::from(qp)));
            }
        });
        kutu(left, crate::tl!("Selection effect color"), |ui| {
            color_row(ui, app.options.selection_color, Tokens::of(crate::tema::active()).selection, "selectionColor", changes);
            if ui.button(crate::tl!("Visual Effect Settings...")).clicked() {
                effects = true;
            }
        });

        kutu(right, crate::tl!("Grip size"), |ui| {
            ui.horizontal(|ui| {
                let mut grip = st.gripsize;
                if ui.add(egui::Slider::new(&mut grip, 1.0..=20.0).integer().suffix(" px")).changed() {
                    changes.insert("GRIPSIZE".into(), json!(grip));
                }
                preview_square(ui, (grip * 2.0) as f32, t.grip);
            });
        });
        kutu(right, crate::tl!("Grips"), |ui| {
            let mut grips = st.grips > 0;
            if ui.checkbox(&mut grips, crate::tl!("Show grips")).changed() {
                changes.insert("GRIPS".into(), json!(u8::from(grips)));
            }
            ui.horizontal(|ui| {
                let mut limit = st.gripobjlimit;
                if ui.add_enabled(st.grips > 0, egui::DragValue::new(&mut limit).range(0..=32_767)).changed() {
                    changes.insert("GRIPOBJLIMIT".into(), json!(limit));
                }
                ui.label(crate::tl!("Object selection limit for display of grips"));
            });
        });
        kutu(right, crate::tl!("Preview"), |ui| {
            ui.label(crate::tl!("Selection preview"));
            let (mut asking, mut idle) = (st.selectionpreview & 2 != 0, st.selectionpreview & 1 != 0);
            let a = ui.checkbox(&mut asking, crate::tl!("When a command is active")).changed();
            let b = ui.checkbox(&mut idle, crate::tl!("When no command is active")).changed();
            if a || b {
                changes.insert("SELECTIONPREVIEW".into(), json!(u8::from(idle) | (u8::from(asking) << 1)));
            }
        });
    });
    if effects {
        visual_effects(ui.ctx(), st, &mut effects, changes);
    }
    ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new(VISUAL_EFFECTS), effects));
}

/// Whether the Visual Effect Settings window is open (egui temp data).
const VISUAL_EFFECTS: &str = "cc_options_visual_effects";

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).strong());
}

/// Visual Effect Settings: the selection area (window and crossing fills) and the objects the
/// cursor never highlights (PREVIEWFILTER). Changes apply at once, as on the Options tabs.
fn visual_effects(ctx: &egui::Context, st: &spcad_engine::Settings, open: &mut bool, changes: &mut Map<String, Value>) {
    let mut close = false;
    egui::Window::new(crate::tl!("Visual Effect Settings"))
        .id(egui::Id::new("cc_visual_effects"))
        .open(open)
        .resizable(false)
        .collapsible(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .show(ctx, |ui| {
            ui.columns(2, |cols| {
                let [left, right] = cols else { return };
                heading(left, crate::tl!("Selection Area Effect"));
                let mut area = st.selectionarea;
                if left.checkbox(&mut area, crate::tl!("Indicate selection area")).changed() {
                    changes.insert("SELECTIONAREA".into(), json!(i32::from(area)));
                }
                left.add_enabled_ui(st.selectionarea, |ui| {
                    ui.label(crate::tl!("Window selection color:"));
                    aci_row(ui, st.windowareacolor, "WINDOWAREACOLOR", changes);
                    ui.label(crate::tl!("Crossing selection color:"));
                    aci_row(ui, st.crossingareacolor, "CROSSINGAREACOLOR", changes);
                    ui.label(crate::tl!("Selection area opacity:"));
                    let mut opacity = st.selectionareaopacity;
                    if ui.add(egui::Slider::new(&mut opacity, 0..=100)).changed() {
                        changes.insert("SELECTIONAREAOPACITY".into(), json!(opacity));
                    }
                });

                heading(right, crate::tl!("Selection Preview Filter"));
                right.label(crate::tl!("Exclude:"));
                for (bit, label) in [
                    (1u8, crate::tl!("Objects on locked layers")),
                    (2, crate::tl!("Xrefs")),
                    (4, crate::tl!("Tables")),
                    (32, crate::tl!("Groups")),
                    (8, crate::tl!("Multiline text")),
                    (16, crate::tl!("Hatches")),
                ] {
                    let mut on = st.previewfilter & bit != 0;
                    if right.checkbox(&mut on, label).changed() {
                        changes.insert("PREVIEWFILTER".into(), json!(if on { st.previewfilter | bit } else { st.previewfilter & !bit }));
                    }
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(crate::tl!("Close")).clicked() {
                        close = true;
                    }
                });
            });
        });
    if close {
        *open = false;
    }
}

/// A colour by its index (ACI 1-255): a swatch and the number; a change goes into `changes`.
fn aci_row(ui: &mut egui::Ui, current: u8, key: &str, changes: &mut Map<String, Value>) {
    ui.horizontal(|ui| {
        let c = spcad_color::aci_rgb(current);
        let (r, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
        ui.painter().rect_filled(r, 2.0, Color32::from_rgb(c.0, c.1, c.2));
        let mut v = current;
        if ui.add(egui::DragValue::new(&mut v).range(1..=255).prefix(format!("{} ", crate::tl!("Color")))).changed() {
            changes.insert(key.into(), json!(v));
        }
    });
}

fn preferences(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>) {
    ui.columns(2, |cols| {
        let [left, right] = cols else { return };
        kutu(left, crate::tl!("Interface"), |ui| {
            grid(ui, "options_preferences_language", |ui| {
                ui.label(crate::tl!("Interface language"));
                let mut lang = app.ui.interface_language;
                egui::ComboBox::from_id_salt("options_language").selected_text(lang.name()).show_ui(ui, |ui| {
                    for p in crate::i18n::Preference::ALL {
                        ui.selectable_value(&mut lang, p, p.name());
                    }
                });
                if lang != app.ui.interface_language {
                    changes.insert("interfaceLanguage".into(), json!(lang.code()));
                }
                ui.end_row();
            });
            let mut timed = app.options.time_sensitive_right_click;
            if ui.checkbox(&mut timed, crate::tl!("Time-sensitive right-click: a quick click is Enter, holding opens the menu")).changed() {
                changes.insert("timeSensitiveRightClick".into(), json!(timed));
            }
            hint(ui, crate::tl!("Turned off, a right-click while a command runs always opens the shortcut menu."));
        });
        kutu(right, crate::tl!("Arrays"), |ui| {
            grid(ui, "options_preferences_array", |ui| {
                ui.label(crate::tl!("Largest array"));
                let mut max = app.session.settings.maxarray;
                if ui
                    .add(egui::DragValue::new(&mut max).range(100..=10_000_000).speed(1000.0).suffix(format!(" {}", crate::tl!("objects"))))
                    .changed()
                {
                    changes.insert("MAXARRAY".into(), json!(max));
                }
                ui.end_row();
            });
            hint(ui, crate::tl!("An array that would create more objects than this is refused."));
        });
    });
}

fn profiles(app: &CadApp, ui: &mut egui::Ui, changes: &mut Map<String, Value>) {
    kutu(ui, crate::tl!("Available profiles"), |ui| {
        ui.label(crate::tl!(
            "SPCAD keeps these preferences, object snap modes and drafting toggles for your user account and restores them when it starts."
        ));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(crate::tl!("Reset to Defaults")).clicked() {
                changes.insert("reset".into(), json!(true));
            }
            if ui.button(crate::tl!("Copy as JSON")).clicked() {
                ui.ctx().copy_text(serde_json::to_string_pretty(&super::report(app)).unwrap_or_default());
            }
        });
        hint(ui, crate::tl!("Reset keeps the interface language."));
    });
}
