use std::io;
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
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

pub fn read_tags(path: PathBuf) -> Option<TagData> {
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
    fs::read_to_string(path).map_or(Vec::new(), |e| parse_tags(e.as_str()))
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
            println!("find tags.db: {}", p.to_str().unwrap());
            read_tag_csv(self, p).unwrap_or(());
        }
        println!("scanned {:?} tags", self.database.len());
    }
}
