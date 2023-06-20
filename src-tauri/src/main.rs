// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![listdir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
