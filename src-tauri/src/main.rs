// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod translate;
mod tagutils;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Runtime, State, Submenu, WindowMenuEvent};
use translate::{TranslateCache, translate};
use crate::tagutils::{QueryTag, TagData, TagHint, TagHintDB, read_tag_db};

#[derive(Default)]
struct CmdState {
    translate_enabled: Mutex<bool>,
    translate_cache: TranslateCache,
    tags_db: TagHintDB,
}

#[tauri::command]
fn listdir(path: &str) -> Vec<TagData> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter_map(tagutils::read_tags)
        .collect::<Vec<_>>()
}

#[tauri::command]
fn save_tags(path: &str, text: &str) -> bool {
    let mut pb = PathBuf::from(path);
    pb.set_extension("txt");
    fs::write(pb, text).is_ok()
}

#[tauri::command]
fn parse_tags(text: &str) -> Vec<String> {
    tagutils::parse_tags(text)
}

#[tauri::command]
async fn translate_tag(text: String, state: State<'_, CmdState>) -> Result<String, ()> {
    Ok(translate(&state.translate_cache, "zh-CN", text.as_str()).await)
}

#[tauri::command]
fn query_tag(text: &str, state: State<'_, CmdState>) -> Vec<QueryTag> {
    let db: &TagHintDB = &state.tags_db;
    let matched = db.search.search(text);
    matched.iter().take(20).map(|tag| {
        let hint = db.database.get(tag).unwrap();
        match hint {
        TagHint::Just(x) => QueryTag {
            tag: tag.clone(),
            suggest: None,
            usage_count: Some(x.clone()),
        },
        TagHint::Alias(x) => QueryTag {
            tag: tag.clone(),
            suggest: Some(x.clone()),
            usage_count: None,
        }
    }
    }).collect::<Vec<_>>()
}

fn window_menu() -> Menu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let open = CustomMenuItem::new("open".to_string(), "Open")
        .accelerator("CmdOrCtrl+O");
    let save = CustomMenuItem::new("save".to_string(), "Save")
        .accelerator("CmdOrCtrl+S");
    let file = Submenu::new("File", Menu::new()
        .add_item(open).add_item(save)
        .add_native_item(MenuItem::Separator).add_native_item(MenuItem::Quit));

    let undo = CustomMenuItem::new("undo".to_string(), "Undo")
        .accelerator("CmdOrCtrl+Z");
    let redo = CustomMenuItem::new("redo".to_string(), "Redo")
        .accelerator("CmdOrCtrl+Shift+Z");
    let edit = Submenu::new("Edit", Menu::new()
        .add_item(undo).add_item(redo));

    let trans = CustomMenuItem::new("translate".to_string(), "Translate tags");
    let view = Submenu::new("View", Menu::new()
        .add_item(trans));

    Menu::new().add_submenu(file).add_submenu(edit).add_submenu(view)
}

fn handle_menu<R: Runtime>(event: WindowMenuEvent<R>) {
    match event.menu_item_id() {
        "quit" => { std::process::exit(0); }
        "open" => { event.window().emit("menu", "open").unwrap(); }
        "save" => { event.window().emit("menu", "save").unwrap(); }
        "undo" => { event.window().emit("menu", "undo").unwrap(); }
        "redo" => { event.window().emit("menu", "redo").unwrap(); }
        "translate" => {
            let state: State<CmdState> = event.window().state();
            let tr = !*state.translate_enabled.lock().unwrap();
            *state.translate_enabled.lock().unwrap() = tr;
            let menu = event.window().menu_handle().get_item("translate");
            menu.set_selected(tr).unwrap();
            event.window().emit("translate", tr).unwrap();
        }
        _ => {}
    }
}

fn main() {
    let mut state: CmdState = CmdState::default();
    state.tags_db = read_tag_db();

    let menu = window_menu();
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            listdir, save_tags, parse_tags, translate_tag, query_tag
        ])
        .menu(menu)
        .on_menu_event(handle_menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
