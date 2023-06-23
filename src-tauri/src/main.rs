// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod translate;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Runtime, State, Submenu, WindowMenuEvent};
use translate::{TranslateCache, translate};

#[derive(Default)]
struct CmdState {
    translate_enabled: Mutex<bool>,
    translate_cache: TranslateCache,
}

#[derive(serde::Serialize)]
struct TagData {
    name: String,
    tags: Vec<String>,
}

#[tauri::command]
fn listdir(path: &str) -> Vec<TagData> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter_map(read_item)
        .collect::<Vec<_>>()
}

#[tauri::command]
fn save_text(path: &str, text: &str) -> bool {
    let mut pb = PathBuf::from(path);
    pb.set_extension("txt");
    fs::write(pb, text).is_ok()
}

#[tauri::command]
async fn translate_tag(text: String, state: State<'_, CmdState>) -> Result<String, ()> {
    Ok(translate(&state.translate_cache, "zh-CN", text.as_str()).await)
}

fn read_item(path: PathBuf) -> Option<TagData> {
    path.extension().and_then(|ext| {
        if ext == "png" || ext == "jpg" {
            path.file_name().map(|e| {
                let name = e.to_os_string().into_string().unwrap();
                TagData {
                    name,
                    tags: get_tags(path.to_owned()),
                }
            })
        } else {
            None
        }
    })
}

fn get_tags(mut path: PathBuf) -> Vec<String> {
    path.set_extension("txt");
    fs::read_to_string(path).map_or(Vec::new(), read_txt_tags)
}

fn read_txt_tags(txt: String) -> Vec<String> {
    enum M {
        Normal,
        Escape,
        White,
    }
    struct S {
        c: M,
        s: String,
        v: Vec<String>,
    }
    let mut s = S {
        c: M::White,
        s: String::new(),
        v: Vec::new(),
    };
    fn rec_str(s: &mut S) {
        s.v.push(s.s.clone());
        s.s.clear();
        s.c = M::White
    }
    for c in txt.chars() {
        match s.c {
            M::Escape => {
                s.s.push(c);
                s.c = M::Normal;
            }
            M::White => match c {
                ' ' => {}
                ',' => rec_str(&mut s),
                _ => {
                    s.s.push(c);
                    s.c = M::Normal;
                }
            },
            M::Normal => match c {
                ' ' => {
                    s.s.push(c);
                    s.c = M::White;
                }
                '\\' => s.c = M::Escape,
                ',' => rec_str(&mut s),
                _ => s.s.push(c),
            },
        }
    }
    s.v
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
    let menu = window_menu();
    tauri::Builder::default()
        .manage(CmdState::default())
        .invoke_handler(tauri::generate_handler![listdir, save_text, translate_tag])
        .menu(menu)
        .on_menu_event(handle_menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
