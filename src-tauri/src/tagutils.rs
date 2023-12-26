use std::io;
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use simsearch::{SimSearch, SearchOptions};

pub enum TagHint {
    Just(u32),
    Alias(String),
}

pub struct TagHintDB {
    pub database: HashMap<String, TagHint>,
    pub search: SimSearch<String>,
}

#[derive(serde::Serialize)]
pub struct TagData {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct QueryTag {
    pub tag: String,
    pub suggest: Option<String>,
    pub usage_count: Option<u32>,
}

impl Default for TagHintDB {
    fn default() -> TagHintDB { TagHintDB::new() }
}

fn is_image_file(ext: &OsStr) -> bool {
    let ext = ext.to_ascii_lowercase();
    ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "webp"
}

pub fn read_tags<P: AsRef<Path> + ToString>(path: P) -> Option<TagData> {
    PathBuf::from(path.as_ref()).extension().and_then(|ext| {
        is_image_file(ext).then_some(TagData {
            name: path.to_string(),
            tags: get_tags(&path),
        })
    })
}

pub fn listdir_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    fs::read_dir(&path).unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter_map(|e|
            if e.is_dir() {
                Some(listdir_files(&e))
            } else if e.is_file() {
                Some(vec![e])
            } else {
                None
            }
        )
        .flatten()
        .collect::<Vec<_>>()
}

pub fn listdir_images<P: AsRef<Path>>(path: P) -> Vec<String> {
    listdir_files(&path).iter()
        .filter_map(|e|
            e.extension().and_then(|ext| {
                is_image_file(&ext).then(|| ()).and(
                    e.strip_prefix(&path).ok().and_then(|e|
                        e.to_str().map(|e| e.to_string())
                    ))
            })
        )
        .collect::<Vec<_>>()
}

fn is_image_exist(images: &HashSet<String>, path: &PathBuf) -> bool {
    path.file_stem().and_then(|e|
        images.contains(e.to_str().unwrap()).then_some(())
    ).is_some()
}

pub fn is_isolated_txt<P: AsRef<Path>>(images: &HashSet<String>, path: P) -> bool {
    let path_buf = PathBuf::from(path.as_ref());
    path_buf.extension().and_then(|ext|
        (ext == "txt" && !is_image_exist(images, &path_buf)).then_some(())
    ).is_some()
}

fn get_tags<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut path_buf = PathBuf::from(path.as_ref());
    path_buf.set_extension("txt");
    fs::read_to_string(path_buf).map_or(Vec::new(), |e| parse_tags(e.as_str()))
}

pub fn parse_tags(txt: &str) -> Vec<String> {
    #[derive(Default)]
    enum M { #[default] White, Normal, Escape }
    #[derive(Default)]
    struct S {
        c: M,
        s: String,
        v: Vec<String>,
    }
    let mut s = S::default();
    impl S {
        fn read_tag(&mut self) {
            if !self.s.is_empty() {
                self.v.push(self.s.clone());
                self.s.clear();
            }
            self.c = M::White;
        }
        fn read_char(&mut self, c: char, m: M) {
            self.s.push(c);
            self.c = m;
        }
    }
    for c in txt.chars() {
        match s.c {
            M::Escape => {
                s.s.push(c);
                s.c = M::Normal;
            }
            M::White => match c {
                ' ' | '_' => {}
                ',' => s.read_tag(),
                _ => s.read_char(c, M::Normal)
            },
            M::Normal => match c {
                ' ' | '_' => s.read_char(' ', M::White),
                '\\' => s.read_char(c, M::Escape),
                ',' => s.read_tag(),
                _ => s.s.push(c),
            },
        }
    }
    s.read_tag();
    s.v
}

fn try_parse_tag(tag: &str) -> String {
    parse_tags(tag).get(0).map_or(tag.to_string(), |e| e.clone())
}

fn read_tag_csv<P: AsRef<Path>>(db: &mut TagHintDB, path: P) -> io::Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false).from_reader(File::open(path)?);
    for result in rdr.records() {
        let record = result?;
        if record.len() < 1 { continue; }
        let tag = try_parse_tag(record.get(0).unwrap());
        match record.len() {
            1 | 2 => { db.database.insert(tag, TagHint::Just(0)); }
            3 => {
                db.database.insert(tag, TagHint::Just(
                    record.get(2).unwrap().parse().unwrap_or(0)));
            }
            4 => {
                db.database.insert(tag.clone(), TagHint::Just(
                    record.get(2).unwrap().parse().unwrap_or(0)));
                for a in parse_tags(record.get(3).unwrap()) {
                    db.database.insert(a, TagHint::Alias(tag.clone()));
                }
            }
            _ => {}
        };
    }
    for rec in db.database.keys() {
        db.search.insert(rec.clone(), rec.as_str());
    }
    Ok(())
}

fn list_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    Ok(dir.filter_map(|e| e.ok().and_then(|e| {
        let p = e.path();
        if p.extension().map_or(false, |x| x == "csv") {
            Some(p)
        } else {
            None
        }
    })).collect::<Vec<_>>())
}

impl TagHintDB {
    pub fn new() -> Self {
        TagHintDB {
            database: HashMap::new(),
            search: SimSearch::new_with(
                SearchOptions::new().stop_whitespace(false)),
        }
    }
    pub fn read_db<P: AsRef<Path>>(&mut self, db_path: P) {
        let csv_list = list_csv(db_path).unwrap_or(Vec::new());
        for p in csv_list {
            println!("load tags.db: {}", p.to_str().unwrap());
            read_tag_csv(self, p).unwrap_or(());
        }
    }
}
