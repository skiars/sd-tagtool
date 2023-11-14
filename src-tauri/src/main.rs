// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod translate;
mod tagutils;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashSet;
use tauri::{AppHandle, CustomMenuItem, Manager, Menu,
            MenuItem, Runtime, State, Submenu, WindowMenuEvent};
use translate::{TranslateCache, translate};
use tagutils::{QueryTag, TagData, TagHint, TagHintDB};

#[derive(Default)]
struct CmdState {
    translate_enabled: Mutex<bool>,
    preview_enabled: Mutex<bool>,
    translate_cache: TranslateCache,
    tags_db: Mutex<TagHintDB>,
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
fn list_isolated_txt(path: &str) -> Vec<String> {
    let images: HashSet<String> = HashSet::from_iter(
        tagutils::listdir_images(path).iter()
            .map(|path|
                path.file_stem().unwrap().to_str().unwrap().to_string()
            )
    );
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter_map(|path| {
            tagutils::is_isolated_txt(&images, path.clone()).then_some(
                path.file_name().unwrap().to_str().unwrap().to_string()
            )
        })
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
async fn translate_tag(text: &str, tl: &str, state: State<'_, CmdState>) -> Result<String, ()> {
    Ok(translate(&state.translate_cache, tl, text).await)
}

#[tauri::command]
fn query_tag(text: &str, state: State<'_, CmdState>) -> Vec<QueryTag> {
    let db = state.tags_db.lock().unwrap();
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

#[tauri::command]
async fn load_tags_db<R: Runtime>(app: AppHandle<R>) -> Result<(), ()> {
    let tags_db_path = app.path_resolver()
        .resolve_resource("shared/tags.db")
        .expect("failed to resolve tags.db path");
    let state: State<CmdState> = app.state();
    let mut db = state.tags_db.lock().unwrap();
    *db = TagHintDB::new(); // clean tag records
    db.read_db(tags_db_path);
    Ok(())
}

#[tauri::command]
fn load_config<R: Runtime>(app: AppHandle<R>) -> Result<serde_json::Value, ()> {
    let resolver = app.path_resolver();
    let path = resolver.app_config_dir().map(
        |e| e.join("config.json")).expect("failed to resolve config path");
    let rdr = fs::File::open(path).or(Err(()))?;
    serde_json::from_reader(rdr).or(Err(()))
}

#[tauri::command]
fn save_config<R: Runtime>(model: serde_json::Value, app: AppHandle<R>) {
    let resolver = app.path_resolver();
    let dir = resolver.app_config_dir().expect("failed to resolve config path");
    fs::create_dir_all(&dir).unwrap_or(());
    fs::File::create(dir.join("config.json")).and_then(|f| {
        serde_json::to_writer_pretty(f, &model).unwrap_or(());
        Ok(())
    }).unwrap_or(());
}

#[tauri::command]
async fn refresh_cache(state: State<'_, CmdState>) -> Result<(), ()> {
    state.translate_cache.lock().await.clear();
    Ok(())
}

fn window_menu() -> Menu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let open = CustomMenuItem::new("open".to_string(), "Open folder")
        .accelerator("CmdOrCtrl+O");
    let save = CustomMenuItem::new("save".to_string(), "Save")
        .accelerator("CmdOrCtrl+S");
    let reload = CustomMenuItem::new("reload".to_string(), "Reload folder")
        .accelerator("CmdOrCtrl+R");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let file = Submenu::new("File", Menu::new()
        .add_item(open).add_item(save).add_item(reload)
        .add_native_item(MenuItem::Separator).add_item(settings)
        .add_native_item(MenuItem::Separator).add_item(quit));

    let undo = CustomMenuItem::new("undo".to_string(), "Undo")
        .accelerator("CmdOrCtrl+Z");
    let redo = CustomMenuItem::new("redo".to_string(), "Redo")
        .accelerator("CmdOrCtrl+Shift+Z");
    let edit = Submenu::new("Edit", Menu::new()
        .add_item(undo).add_item(redo));

    let trans = CustomMenuItem::new("translate".to_string(), "Translate tags");
    let preview = CustomMenuItem::new("preview".to_string(), "Image Preview");
    let view = Submenu::new("View", Menu::new()
        .add_item(trans).add_item(preview));

    let delete_txt = CustomMenuItem::new("delete_txt".to_string(), "Delete isolated txt");
    let tools = Submenu::new("Tools", Menu::new().add_item(delete_txt));

    Menu::new().add_submenu(file).add_submenu(edit).add_submenu(view).add_submenu(tools)
}

fn handle_menu<R: Runtime>(event: WindowMenuEvent<R>) {
    match event.menu_item_id() {
        "quit" => { event.window().emit("menu", "quit").unwrap(); }
        "open" => { event.window().emit("menu", "open").unwrap(); }
        "save" => { event.window().emit("menu", "save").unwrap(); }
        "reload" => { event.window().emit("menu", "reload").unwrap(); }
        "undo" => { event.window().emit("menu", "undo").unwrap(); }
        "redo" => { event.window().emit("menu", "redo").unwrap(); }
        "settings" => { event.window().emit("menu", "settings").unwrap(); }
        "delete_txt" => { event.window().emit("menu", "delete_txt").unwrap(); }
        "translate" => {
            let state: State<CmdState> = event.window().state();
            let tr = !*state.translate_enabled.lock().unwrap();
            *state.translate_enabled.lock().unwrap() = tr;
            let menu = event.window().menu_handle().get_item("translate");
            menu.set_selected(tr).unwrap();
            event.window().emit("translate", tr).unwrap();
        }
        "preview" => {
            let state: State<CmdState> = event.window().state();
            let s = !*state.preview_enabled.lock().unwrap();
            *state.preview_enabled.lock().unwrap() = s;
            let menu = event.window().menu_handle().get_item("preview");
            menu.set_selected(s).unwrap();
            event.window().emit("preview", s).unwrap();
        }
        _ => {}
    }
}

fn main() {
    let menu = window_menu();
    tauri::Builder::default()
        .manage(CmdState::default())
        .invoke_handler(tauri::generate_handler![
            listdir, save_tags, parse_tags, translate_tag, query_tag, load_tags_db,
            load_config, save_config, refresh_cache, list_isolated_txt
        ])
        .menu(menu)
        .on_menu_event(handle_menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
