//! The floating command line: history, the active prompt with clickable keywords, typed input
//! (keyboard focus stays on the drawing — typing anywhere goes here) and AutoComplete.

use std::collections::HashSet;

use egui::{Key, Pos2, Rect, Sense, Stroke, pos2, vec2};
use spcad_engine::Input;

use crate::CadApp;
use crate::tema::Tokens;

#[derive(Default)]
pub struct CmdLine {
    pub buffer: String,
    /// The selection as of the last frame, to notice when it changes.
    seen_selection: Vec<spcad_doc::Handle>,
    /// Something was typed at the command line since the selection was made.
    typed_since_selection: bool,
    pub history: Vec<String>,
    pub history_pos: Option<usize>,
    /// What was typed at command prompts (not command names), oldest first: the in-command
    /// shortcut menu's Recent Input.
    pub recent_input: Vec<String>,
    /// The expanded history window is open (F2, the chevron, or a click on the history lines).
    pub expanded: bool,
    /// The Dynamic Input options menu at the cursor (Down Arrow at a prompt with keywords).
    pub options: Option<crate::dinamik_girdi::OptionsMenu>,
    /// The text whose second Dynamic Input value Tab locked; the lock holds while it is unchanged.
    locked_second: Option<String>,
    /// Where the options menu's rows were drawn last frame (screen), for clicks on them.
    pub option_rows: Vec<(Rect, String)>,
    /// Where the Dynamic Input tooltip stays while the options menu is open.
    pub options_anchor: Option<Pos2>,
}

/// The most history lines the command line shows above its bar.
pub const MAX_LINES: usize = 12;
const LINE_H: f32 = 18.0;

/// The Dynamic Input frame for the prompt on screen, or `None` when the value boxes are not live
/// (Dynamic Input off, no point prompt or a pending selection window).
pub fn dyn_frame(app: &CadApp) -> Option<crate::dinamik_girdi::Frame> {
    let st = &app.session.settings;
    if !st.dynmode || app.session.pending_window.is_some() {
        return None;
    }
    let p = app.session.current_prompt().filter(spcad_engine::nokta_degistiricileri::accepts)?;
    Some(crate::dinamik_girdi::Frame { base: p.base, cartesian: st.dynpi_cartesian, absolute: st.dynpi_absolute })
}

/// Take Tab away from egui before it moves keyboard focus to the next widget (which would leave
/// the command line deaf to typing). While no widget has focus, a pressed Tab becomes a `"\t"`
/// text event, which keeps its place among the other keystrokes; [`keyboard`] handles it.
pub fn capture_tab(ctx: &egui::Context, raw: &mut egui::RawInput) {
    if ctx.egui_wants_keyboard_input() {
        return;
    }
    raw.events.retain_mut(|e| {
        let tab = match e {
            egui::Event::Key { key: Key::Tab, pressed, .. } => Some(*pressed),
            _ => None,
        };
        match tab {
            Some(true) => {
                *e = egui::Event::Text("\t".into());
                true
            }
            Some(false) => false,
            None => true,
        }
    });
}

/// Tab on the command line: cycle the object snaps under the cursor, type the Dynamic Input
/// separator (`,` or `<`) after a value, or complete a command name.
fn tab(app: &mut CadApp) {
    // Grip dimensional input: Tab locks the typed value and moves to the next field.
    if crate::tutamac_dinamik::tab(app) {
        return;
    }
    // Over a snap marker with nothing typed, Tab cycles the candidate snaps.
    if app.cmd.buffer.is_empty() && app.canvas.snap.is_some() {
        app.canvas.snap_tab = true;
    } else if let Some(f) = dyn_frame(app)
        && let Some(sep) = crate::dinamik_girdi::tab_separator(&app.cmd.buffer, &f)
    {
        app.cmd.buffer.push(sep);
    } else if let Some(f) = dyn_frame(app)
        && crate::dinamik_girdi::can_lock_second(&app.cmd.buffer, &f)
    {
        // Both values typed: Tab locks the second one as well.
        app.cmd.locked_second = Some(app.cmd.buffer.clone());
    } else if let Some(s) = suggestions(&app.cmd.buffer).first() {
        app.cmd.buffer = s.id.clone();
    }
}

/// An AutoComplete suggestion for the command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Suggestion {
    /// Command identifier (uppercase, e.g. "RECTANG"), used for buffer completion and execution.
    pub id: String,
    /// Display text shown in the AutoComplete list (e.g. "REC (RECTANG)" or "RECOVER").
    pub display: String,
    /// Human-readable command label (e.g. "Rectangle").
    pub label: &'static str,
}

/// Matching command names/aliases for AutoComplete.
fn suggestions(prefix: &str) -> Vec<Suggestion> {
    if prefix.is_empty() || prefix.contains(' ') {
        return Vec::new();
    }
    let p = prefix.to_ascii_lowercase();

    let mut out: Vec<Suggestion> = Vec::new();
    let mut seen_ids: HashSet<&'static str> = HashSet::new();

    // 1. Exact match: alias resolution or exact command name match.
    let resolved = spcad_engine::komut::resolve_alias(&p);
    if let Some(spec) = spcad_engine::komut::find_command(&resolved)
        && !spec.id.contains('.')
    {
        let display =
            if spec.id == p { spec.id.to_ascii_uppercase() } else { format!("{} ({})", p.to_ascii_uppercase(), spec.id.to_ascii_uppercase()) };
        out.push(Suggestion { id: spec.id.to_ascii_uppercase(), display, label: spec.label });
        seen_ids.insert(spec.id);
    }

    // 2. Command ID prefix matches (excluding already matched exact command).
    let mut id_matches: Vec<&'static spcad_engine::CommandSpec> =
        spcad_engine::command_specs().iter().filter(|c| !c.id.contains('.') && !seen_ids.contains(c.id) && c.id.starts_with(&p)).collect();
    id_matches.sort_by_key(|c| (c.id.len(), c.id));
    for c in id_matches {
        out.push(Suggestion { id: c.id.to_ascii_uppercase(), display: c.id.to_ascii_uppercase(), label: c.label });
        seen_ids.insert(c.id);
    }

    // 3. Command alias prefix matches (for commands not yet added).
    let mut alias_matches: Vec<(&'static str, &'static spcad_engine::CommandSpec)> = Vec::new();
    for c in spcad_engine::command_specs() {
        if c.id.contains('.') || seen_ids.contains(c.id) {
            continue;
        }
        let matching_alias = c.aliases.iter().copied().filter(|a| a.starts_with(&p)).min_by_key(|a| (a.len(), *a));
        if let Some(alias) = matching_alias {
            alias_matches.push((alias, c));
        }
    }
    alias_matches.sort_by_key(|(alias, c)| (alias.len(), *alias, c.id));
    for (alias, c) in alias_matches {
        if seen_ids.insert(c.id) {
            out.push(Suggestion {
                id: c.id.to_ascii_uppercase(),
                display: format!("{} ({})", alias.to_ascii_uppercase(), c.id.to_ascii_uppercase()),
                label: c.label,
            });
        }
    }

    out.truncate(8);
    out
}

impl CmdLine {
    /// Track the selection: a new selection starts with nothing typed since.
    pub fn watch_selection(&mut self, sel: &[spcad_doc::Handle]) {
        if self.seen_selection != sel {
            self.seen_selection = sel.to_vec();
            self.typed_since_selection = false;
        }
    }

    /// Note that the command line holds typed text.
    pub fn note_typing(&mut self) {
        if !self.buffer.is_empty() {
            self.typed_since_selection = true;
        }
    }

    /// Tab locked the second Dynamic Input value and nothing was typed since.
    pub fn second_locked(&self) -> bool {
        self.locked_second.as_deref() == Some(self.buffer.as_str())
    }

    /// The options menu row under `at` (screen), if the menu is open.
    pub fn option_at(&self, at: Pos2) -> Option<String> {
        self.options.as_ref()?;
        self.option_rows.iter().find(|(r, _)| r.contains(at)).map(|(_, k)| k.clone())
    }
}

/// Keep the options menu only while its prompt is on screen with Dynamic Input and DYNPROMPT on.
fn options_still_fit(app: &mut CadApp) {
    let st = &app.session.settings;
    let on = st.dynmode && st.dynprompt && app.canvas.hot_grips.is_empty();
    let kws = app.session.current_prompt().map(|p| p.keywords).unwrap_or_default();
    if app.cmd.options.as_ref().is_some_and(|m| !on || !m.fits(&kws)) {
        app.cmd.options = None;
    }
}

/// Down Arrow at a prompt with keywords (Dynamic Input on, DYNPROMPT on, not browsing the
/// history): open the options menu at the cursor, or move its highlight. `false` when the key is
/// left for the history.
fn options_key(app: &mut CadApp, step: isize) -> bool {
    if let Some(m) = app.cmd.options.as_mut() {
        m.step(step);
        return true;
    }
    let st = &app.session.settings;
    if step < 0 || !st.dynmode || !st.dynprompt || app.cmd.history_pos.is_some() || !app.canvas.hot_grips.is_empty() {
        return false;
    }
    let kws = app.session.current_prompt().map(|p| p.keywords).unwrap_or_default();
    app.cmd.options = crate::dinamik_girdi::OptionsMenu::open(&kws);
    app.cmd.options.is_some()
}

/// Send a keyword picked from the options menu, as a click on it at the command line does.
pub fn pick_option(app: &mut CadApp, keyword: String) {
    app.cmd.options = None;
    app.cmd.buffer.clear();
    if let Err(e) = app.session.input(Input::Keyword(keyword)) {
        app.session.echo(e.to_string());
    }
}

/// Whether Backspace should erase the selection instead of editing the command line. The key
/// labelled "delete" on a Mac sends Backspace, so on macOS it erases selected objects, but only
/// when it can't be meant for the command line: a fresh, plain press (never an auto-repeat, so
/// holding it to clear typing can't run on into the drawing, and not Option/Cmd+delete, which
/// delete a word or line of text on macOS), no command running, the command line empty, and
/// nothing typed there since the selection was made.
pub fn backspace_erases(mac: bool, press: KeyPress, buffer: &str, command_running: bool, has_selection: bool, typed_since_selection: bool) -> bool {
    mac && press.plain && !press.repeat && buffer.is_empty() && !command_running && has_selection && !typed_since_selection
}

/// How a key was pressed.
#[derive(Clone, Copy, Debug)]
pub struct KeyPress {
    /// The operating system's auto-repeat while the key is held.
    pub repeat: bool,
    /// No Option, Control or Command held (Shift doesn't matter).
    pub plain: bool,
}

/// Handle keyboard input destined for the command line (when no text field has focus).
pub fn keyboard(app: &mut CadApp, ctx: &egui::Context) {
    if ctx.egui_wants_keyboard_input() {
        return;
    }
    let events = ctx.input(|i| i.events.clone());
    let selection = app.session.selection();
    app.cmd.watch_selection(&selection);
    let mac = ctx.os() == egui::os::OperatingSystem::Mac;
    let text_prompt = app.session.current_prompt().is_some_and(|p| p.accept.text && !p.accept.point && !p.accept.number);
    options_still_fit(app);
    for ev in events {
        // The options menu takes the arrows, Enter and Escape; typing closes it.
        if app.cmd.options.is_some() {
            let space = matches!(&ev, egui::Event::Text(t) if t == " ") && !text_prompt;
            match &ev {
                _ if space => {
                    if let Some(k) = app.cmd.options.as_ref().and_then(|m| m.current()).map(str::to_owned) {
                        pick_option(app, k);
                    }
                    continue;
                }
                egui::Event::Key { key: Key::Enter, pressed: true, .. } => {
                    if let Some(k) = app.cmd.options.as_ref().and_then(|m| m.current()).map(str::to_owned) {
                        pick_option(app, k);
                    }
                    continue;
                }
                egui::Event::Key { key: Key::Escape, pressed: true, .. } => {
                    app.cmd.options = None;
                    continue;
                }
                egui::Event::Key { key: Key::ArrowUp, pressed: true, modifiers, .. } if !modifiers.any() => {
                    options_key(app, -1);
                    continue;
                }
                egui::Event::Key { key: Key::ArrowDown | Key::ArrowUp, pressed: true, .. } => {}
                // Space arrives as a key and as text; the text picks.
                egui::Event::Key { key: Key::Space, .. } => {}
                egui::Event::Text(_) | egui::Event::Key { pressed: true, .. } => app.cmd.options = None,
                _ => {}
            }
        }
        match ev {
            egui::Event::Text(t) => {
                if t == "\t" {
                    tab(app);
                } else if t == " " && !text_prompt {
                    submit(app);
                } else {
                    app.cmd.buffer.push_str(&t);
                }
            }
            egui::Event::Key { key, pressed: true, repeat, modifiers, .. } => match key {
                Key::Enter => submit(app),
                Key::Escape => {
                    app.cmd.buffer.clear();
                    app.session.cancel();
                }
                Key::Backspace => {
                    let press = KeyPress { repeat, plain: !(modifiers.alt || modifiers.ctrl || modifiers.command) };
                    let erase = backspace_erases(
                        mac,
                        press,
                        &app.cmd.buffer,
                        app.session.prompting(),
                        !app.session.selection().is_empty(),
                        app.cmd.typed_since_selection,
                    );
                    if erase {
                        let _ = app.run("erase.selection", serde_json::json!({}));
                    } else {
                        app.cmd.buffer.pop();
                    }
                }
                Key::ArrowUp if !modifiers.any() => {
                    if !app.cmd.history.is_empty() {
                        let n = app.cmd.history.len();
                        let pos = app.cmd.history_pos.map(|p| p.saturating_sub(1)).unwrap_or(n - 1);
                        app.cmd.history_pos = Some(pos);
                        app.cmd.buffer = app.cmd.history.get(pos).cloned().unwrap_or_default();
                    }
                }
                Key::ArrowDown if !modifiers.any() => {
                    if !options_key(app, 1)
                        && let Some(p) = app.cmd.history_pos
                    {
                        let np = p + 1;
                        if np >= app.cmd.history.len() {
                            app.cmd.history_pos = None;
                            app.cmd.buffer.clear();
                        } else {
                            app.cmd.history_pos = Some(np);
                            app.cmd.buffer = app.cmd.history.get(np).cloned().unwrap_or_default();
                        }
                    }
                }
                Key::Tab => tab(app),
                Key::Delete if app.cmd.buffer.is_empty() && !app.session.prompting() => {
                    let _ = app.run("erase.selection", serde_json::json!({}));
                }
                _ => {}
            },
            _ => {}
        }
        app.cmd.note_typing();
    }
}

pub fn submit(app: &mut CadApp) {
    // Grip dimensional input: the typed lengths and angles place the grip.
    if crate::tutamac_dinamik::submit(app) {
        return;
    }
    // Dynamic Input: a split entry (`40,30`, `40<90`, `40,`) is completed from the cursor and
    // measured from the last point.
    if let Some(f) = dyn_frame(app) {
        let cursor = app.canvas.cursor.unwrap_or(app.session.cursor);
        let last = app.session.last_point;
        let done = crate::dinamik_girdi::parse(&app.cmd.buffer, &f).and_then(|e| crate::dinamik_girdi::complete(&e, &f, cursor, last));
        if let Some(text) = done {
            app.cmd.buffer = text;
        }
    }
    let text = std::mem::take(&mut app.cmd.buffer);
    app.cmd.locked_second = None;
    app.cmd.history_pos = None;
    if !text.trim().is_empty() && !app.session.prompting() {
        app.cmd.history.push(text.clone());
        if app.cmd.history.len() > 200 {
            app.cmd.history.remove(0);
        }
    }
    crate::baglam_menusu::note_input(app, &text);
    if text.is_empty() {
        if let Err(e) = app.session.input(Input::Enter) {
            app.session.echo(e.to_string());
        }
        return;
    }
    app.cmdline(&text);
}

/// The history lines shown above the input: the user's preference, the cap, and what the log has.
fn visible_lines(app: &CadApp) -> usize {
    app.ui.history_lines.min(MAX_LINES).min(app.session.log.len())
}

/// Height of the history part of the docked command line.
fn history_height(app: &CadApp) -> f32 {
    if app.cmd.expanded {
        let lines = app.session.log.len().max(3);
        return (LINE_H * lines as f32 + 4.0).clamp(3.0 * LINE_H, MAX_LINES as f32 * LINE_H * 2.0);
    }
    visible_lines(app) as f32 * LINE_H + 2.0
}

const BAR_H: f32 = 34.0;

/// The docked command line at the bottom of the window: history above, the prompt and typed text
/// below, and the AutoComplete list over the drawing area.
pub fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let h = history_height(app) + BAR_H + 6.0;
    egui::Panel::bottom("cc_cmdline").exact_size(h).frame(egui::Frame::NONE.fill(t.cmd_bg)).show(ui, |ui| {
        let r = ui.max_rect();
        let p = ui.painter().clone();
        p.hline(r.x_range(), r.top(), Stroke::new(1.0, t.cmd_border));
        let bar = Rect::from_min_max(pos2(r.left() + 6.0, r.bottom() - BAR_H - 3.0), pos2(r.right() - 6.0, r.bottom() - 3.0));
        p.rect_filled(bar, 4.0, t.field);
        p.rect_stroke(bar, 4.0, Stroke::new(1.0, t.cmd_border), egui::StrokeKind::Inside);
        // The top edge resizes the inline history.
        let grip = Rect::from_min_max(pos2(r.left(), r.top()), pos2(r.right(), r.top() + 4.0));
        let resp = ui.interact(grip, ui.id().with("cmdline_grip"), Sense::drag());
        if resp.hovered() || resp.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
        }
        if resp.dragged()
            && let Some(pointer) = ui.input(|i| i.pointer.interact_pos())
        {
            let n = ((r.bottom() - BAR_H - 3.0 - pointer.y) / LINE_H).round().clamp(0.0, MAX_LINES as f32) as usize;
            if n != app.ui.history_lines && !app.cmd.expanded {
                let _ = app.run("ui.cmdline.lines", serde_json::json!({ "lines": n }));
            }
        }
        // History lines above the input.
        let hist = Rect::from_min_max(pos2(bar.left(), r.top() + 4.0), pos2(bar.right(), bar.top() - 3.0));
        if app.cmd.expanded {
            history_window(app, ui, hist);
        } else {
            let lines: Vec<String> = app.session.log.iter().rev().take(visible_lines(app)).rev().cloned().collect();
            for (i, l) in lines.iter().enumerate() {
                p.with_clip_rect(hist).text(
                    pos2(hist.left() + 4.0, hist.top() + 1.0 + LINE_H * i as f32),
                    egui::Align2::LEFT_TOP,
                    l,
                    crate::tema::mono(),
                    t.text_dim,
                );
            }
        }
        // Prompt, keywords and typed text.
        let mut x = bar.left() + 6.0;
        let font = crate::tema::mono();
        let prompt = app.session.current_prompt();
        match &prompt {
            Some(pr) => {
                let name = app.session.running.as_ref().filter(|r| r.id != "grip").map(|r| r.id.to_ascii_uppercase() + " ").unwrap_or_default();
                let g = p.layout_no_wrap(name, font.clone(), t.text_faint);
                let gw = g.size().x;
                p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text_faint);
                x += gw;
                let message = if pr.raw { pr.display() } else { crate::i18n::t(&pr.message).to_string() };
                let g = p.layout_no_wrap(message, font.clone(), t.text);
                let gw = g.size().x;
                p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                x += gw;
                if !pr.raw && !pr.keywords.is_empty() {
                    let g = p.layout_no_wrap(if pr.message.is_empty() { " [".into() } else { crate::tl!(" or [").into() }, font.clone(), t.text);
                    let gw = g.size().x;
                    p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                    x += gw;
                    let kws = pr.keywords.clone();
                    for (i, k) in kws.iter().enumerate() {
                        let g = p.layout_no_wrap(k.clone(), font.clone(), t.cmd_keyword);
                        let rr = Rect::from_min_size(pos2(x, bar.top() + 3.0), vec2(g.size().x, bar.height() - 6.0));
                        let resp = ui.interact(rr, ui.id().with(("kw", i)), Sense::click());
                        if resp.hovered() {
                            p.rect_filled(rr, 2.0, t.cmd_keyword_hover);
                        }
                        let gw = g.size().x;
                        p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.cmd_keyword);
                        x += gw;
                        if resp.clicked() {
                            let _ = app.session.input(Input::Keyword(k.clone()));
                        }
                        if i + 1 < kws.len() {
                            let g = p.layout_no_wrap("/".into(), font.clone(), t.text);
                            let gw = g.size().x;
                            p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                            x += gw;
                        }
                    }
                    let g = p.layout_no_wrap("]".into(), font.clone(), t.text);
                    let gw = g.size().x;
                    p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                    x += gw;
                }
                if let Some(d) = pr.default.as_ref().filter(|_| !pr.raw) {
                    let g = p.layout_no_wrap(format!(" <{d}>"), font.clone(), t.text);
                    let gw = g.size().x;
                    p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                    x += gw;
                }
                if !pr.raw {
                    let g = p.layout_no_wrap(": ".into(), font.clone(), t.text);
                    let gw = g.size().x;
                    p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
                    x += gw;
                } else if !pr.message.ends_with(' ') {
                    x += 4.0;
                }
            }
            None => {
                let g = p.layout_no_wrap(crate::tl!("Command:").to_string(), font.clone(), t.text_faint);
                let gw = g.size().x;
                p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text_faint);
                x += gw + 6.0;
                if app.cmd.buffer.is_empty() {
                    let g = p.layout_no_wrap(crate::tl!("Type a command, or press Ctrl+K").to_string(), font.clone(), t.text_faint);
                    p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text_faint);
                }
            }
        }
        let g = p.layout_no_wrap(app.cmd.buffer.clone(), font, t.text);
        let gw = g.size().x;
        p.galley(pos2(x, bar.center().y - g.size().y / 2.0), g, t.text);
        let blink = (ui.input(|i| i.time) * 2.0).floor() as i64 % 2 == 0;
        if blink && !app.cmd.expanded {
            p.line_segment([pos2(x + gw + 1.0, bar.top() + 5.0), pos2(x + gw + 1.0, bar.bottom() - 5.0)], Stroke::new(1.0, t.text));
        }
        // The chevron opens and closes the full history (F2 does the same).
        let cr = Rect::from_center_size(pos2(bar.right() - 12.0, bar.center().y), vec2(20.0, 20.0));
        let cresp = ui.interact(cr, ui.id().with("cmdline_chevron"), Sense::click());
        crate::simgeler::paint(
            &p,
            Rect::from_center_size(cr.center(), vec2(14.0, 14.0)),
            if app.cmd.expanded { crate::simgeler::Icon::ChevronDown } else { crate::simgeler::Icon::ChevronUp },
            false,
        );
        crate::simgeler::describe_control(ui, &cresp, "Command History", Some(app.cmd.expanded), false);
        if cresp.clone().on_hover_text(crate::tl!("Command History")).clicked() {
            let _ = app.run("ui.toggle.history", serde_json::json!({}));
        }
        crate::kisayol_menuleri::attach(app, &cresp, crate::kisayol_menuleri::command_line_entries);
        // AutoComplete list above the bar.
        if !app.session.prompting() && !app.cmd.expanded {
            let sug = suggestions(&app.cmd.buffer);
            if !sug.is_empty() {
                let rh = 20.0;
                let lr = Rect::from_min_size(
                    pos2(bar.left() + 24.0, bar.top() - 4.0 - rh * sug.len() as f32),
                    vec2((bar.width() - 24.0).max(0.0), rh * sug.len() as f32),
                );
                p.rect_filled(lr, 3.0, t.list_bg);
                p.rect_stroke(lr, 3.0, Stroke::new(1.0, t.cmd_border), egui::StrokeKind::Inside);
                for (i, s) in sug.iter().enumerate() {
                    let rr = Rect::from_min_size(pos2(lr.left(), lr.top() + rh * i as f32), vec2(lr.width(), rh));
                    let resp = ui.interact(rr, ui.id().with(("sug", i)), Sense::click());
                    if resp.hovered() || i == 0 {
                        p.rect_filled(rr.shrink(1.0), 2.0, if resp.hovered() { t.cmd_keyword_hover } else { t.list_row });
                    }
                    p.text(pos2(rr.left() + 8.0, rr.center().y), egui::Align2::LEFT_CENTER, &s.display, crate::tema::body(), t.text);
                    p.text(
                        pos2(rr.right() - 8.0, rr.center().y),
                        egui::Align2::RIGHT_CENTER,
                        crate::i18n::t(s.label),
                        crate::tema::small(),
                        t.text_faint,
                    );
                    if resp.clicked() {
                        app.cmd.buffer.clear();
                        app.start(&s.id.to_ascii_lowercase());
                    }
                }
            }
        }
    });
}

/// The expanded history: the whole command log, scrollable, newest at the bottom.
fn history_window(app: &CadApp, ui: &mut egui::Ui, r: Rect) {
    let t = Tokens::get();
    let inner = r.shrink2(vec2(4.0, 0.0));
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::top_down(egui::Align::Min)));
    child.spacing_mut().item_spacing.y = 0.0;
    let log = &app.session.log;
    egui::ScrollArea::vertical().id_salt("cmd_history").stick_to_bottom(true).auto_shrink([false, false]).show_rows(
        &mut child,
        LINE_H,
        log.len(),
        |ui, rows| {
            for l in log.get(rows).into_iter().flatten() {
                let (row, _) = ui.allocate_exact_size(vec2(ui.available_width(), LINE_H), Sense::hover());
                let p = ui.painter().with_clip_rect(row.intersect(ui.clip_rect()));
                p.text(pos2(row.left(), row.center().y), egui::Align2::LEFT_CENTER, l, crate::tema::mono(), t.text_dim);
            }
        },
    );
}

#[cfg(test)]
mod testler {
    use super::*;
    use egui::{Event, Pos2};
    use spcad_engine::Session;

    fn frame(app: &mut CadApp, ctx: &egui::Context, events: Vec<Event>) {
        let raw = egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1400.0, 900.0))), events, ..Default::default() };
        let mut out = ctx.run_ui(raw, |ui| {
            app.logic(ui.ctx());
            app.ui(ui);
        });
        out.textures_delta.clear();
    }

    #[test]
    fn docked_command_line_shows_the_requested_history_lines() {
        let mut app = CadApp::new(Session::empty(), crate::Services::default());
        for i in 0..20 {
            app.session.echo(format!("line {i}"));
        }
        app.ui.history_lines = 5;
        assert_eq!(visible_lines(&app), 5);
        // The cap and the log bound the preference.
        app.ui.history_lines = 99;
        assert_eq!(visible_lines(&app), MAX_LINES.min(20));
        // The expanded history has room for the log, within its bounds.
        app.cmd.expanded = true;
        assert!(history_height(&app) >= 3.0 * LINE_H);
        app.cmd.expanded = false;
        assert_eq!(history_height(&app), MAX_LINES as f32 * LINE_H + 2.0);
    }

    #[test]
    fn command_line_is_docked_above_the_status_bar_and_f2_expands_it() {
        let mut app = CadApp::new(Session::empty(), crate::Services::default());
        app.start("ui.sample");
        for i in 0..20 {
            app.session.echo(format!("line {i}"));
        }
        let ctx = egui::Context::default();
        frame(&mut app, &ctx, Vec::new());
        frame(&mut app, &ctx, Vec::new());
        // The drawing area ends above the command line, the layout tabs and the status bar.
        let canvas = app.canvas.rect.unwrap_or(Rect::NOTHING);
        assert!(canvas.bottom() < 900.0 - 50.0, "command line, layout tabs and status bar take the bottom");
        // The docked command line never puts its controls over the drawing area.
        assert!(canvas.top() >= 0.0);
        let f2 = |pressed| Event::Key { key: Key::F2, physical_key: None, pressed, repeat: false, modifiers: egui::Modifiers::NONE };
        frame(&mut app, &ctx, vec![f2(true), f2(false)]);
        assert!(app.cmd.expanded);
        assert!(app.session.pending_window.is_none());
        assert_eq!(app.run("ui.toggle.history", serde_json::json!({"on": false})), Ok(serde_json::json!({"on": false})));
        app.cmdline("textscr");
        assert!(app.cmd.expanded);
        let _ = app.run("ui.toggle.history", serde_json::json!({}));
        frame(&mut app, &ctx, Vec::new());
        assert!(!app.cmd.expanded);
        assert_eq!(app.run("ui.cmdline.lines", serde_json::json!({"lines": 99})), Ok(serde_json::json!({"lines": MAX_LINES})));
    }
}

#[cfg(test)]
mod delete_key_tests {
    use super::*;
    use spcad_doc::Handle;

    const PRESS: KeyPress = KeyPress { repeat: false, plain: true };

    #[test]
    fn mac_delete_erases_a_fresh_selection() {
        assert!(backspace_erases(true, PRESS, "", false, true, false));
        // Not on Windows/Linux, where Backspace never deletes objects.
        assert!(!backspace_erases(false, PRESS, "", false, true, false));
        // Nothing selected, a command running, or text on the command line: just edit text.
        assert!(!backspace_erases(true, PRESS, "", false, false, false));
        assert!(!backspace_erases(true, PRESS, "", true, true, false));
        assert!(!backspace_erases(true, PRESS, "m", false, true, false));
        // Option/Cmd+delete are text-editing gestures on macOS.
        assert!(!backspace_erases(true, KeyPress { plain: false, ..PRESS }, "", false, true, false));
    }

    #[test]
    fn holding_delete_to_clear_typing_never_erases() {
        // Auto-repeat after the last character is gone.
        assert!(!backspace_erases(true, KeyPress { repeat: true, ..PRESS }, "", false, true, false));
        // Typed, then cleared with separate presses: still not erased.
        assert!(!backspace_erases(true, PRESS, "", false, true, true));
    }

    #[test]
    fn typing_is_forgotten_when_the_selection_changes() {
        let mut c = CmdLine::default();
        c.watch_selection(&[Handle(1)]);
        c.buffer.push('m');
        c.note_typing();
        c.buffer.clear();
        c.watch_selection(&[Handle(1)]);
        assert!(c.typed_since_selection);
        c.watch_selection(&[Handle(1), Handle(2)]);
        assert!(!c.typed_since_selection);
    }
}

#[cfg(test)]
mod autocomplete_tests {
    use super::*;

    #[test]
    fn exact_alias_is_first_suggestion_with_alias_display() {
        let rec = suggestions("rec");
        assert!(!rec.is_empty());
        assert_eq!(rec[0].id, "RECTANG");
        assert_eq!(rec[0].display, "REC (RECTANG)");
        assert_eq!(rec[0].label, "Rectangle");
        // RECOVER should follow RECTANG, not precede it
        assert!(rec.iter().any(|s| s.id == "RECOVER"));
        assert_eq!(rec[1].id, "RECOVER");
        assert_eq!(rec[1].display, "RECOVER");

        let l = suggestions("l");
        assert_eq!(l[0].id, "LINE");
        assert_eq!(l[0].display, "L (LINE)");

        let c = suggestions("c");
        assert_eq!(c[0].id, "CIRCLE");
        assert_eq!(c[0].display, "C (CIRCLE)");

        let co = suggestions("co");
        assert_eq!(co[0].id, "COPY");
        assert_eq!(co[0].display, "CO (COPY)");
    }

    #[test]
    fn prefix_without_alias_shows_command_id() {
        let rect = suggestions("rect");
        assert_eq!(rect[0].id, "RECTANG");
        assert_eq!(rect[0].display, "RECTANG");

        let line = suggestions("line");
        assert_eq!(line[0].id, "LINE");
        assert_eq!(line[0].display, "LINE");
    }

    #[test]
    fn tab_autocompletes_alias_to_resolved_command() {
        let mut app = CadApp::new(spcad_engine::Session::empty(), crate::Services::default());
        app.cmd.buffer = "rec".to_string();
        tab(&mut app);
        assert_eq!(app.cmd.buffer, "RECTANG");
    }
}
