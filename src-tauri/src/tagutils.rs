use std::io;
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::collections::BTreeMap;

pub enum TagHint {
    Just(u32),
    Alias(String)
}

pub type TagHintDB = BTreeMap<String, TagHint>;

#[derive(serde::Serialize)]
pub struct TagData {
    name: String,
    tags: Vec<String>
}

#[derive(serde::Serialize)]
pub struct QueryTag {
    pub tag: String,
    pub suggest: Option<String>,
    pub usage_count: Option<u32>
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

fn read_tag_csv<P: AsRef<Path>>(db: &mut TagHintDB, path: P) -> io::Result<()> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    for result in rdr.records() {
        let record = result?;
        if record.len() < 1 { continue; }
        let tag = record.get(0).unwrap().to_string();
        match record.len() {
            1 | 2 => { db.insert(tag, TagHint::Just(0)); }
            3 => {
                db.insert(tag, TagHint::Just(
                    record.get(2).unwrap().parse().unwrap_or(0)));
            }
            4 => {
                db.insert(tag.clone(), TagHint::Just(
                    record.get(2).unwrap().parse().unwrap_or(0)));
                for a in parse_tags(record.get(3).unwrap()) {
                    db.insert(a, TagHint::Alias(tag.clone()));
                }
            }
            _ => {}
        };
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

pub fn read_tag_db() -> TagHintDB {
    match env::current_exe() {
        Ok(mut exe_path) => {
            exe_path.set_file_name("tags.db");
            let mut db: TagHintDB = TagHintDB::new();
            let csv_list = list_csv(exe_path).unwrap_or(Vec::new());
            for p in csv_list {
                println!("find tags.db: {}", p.to_str().unwrap());
                read_tag_csv(&mut db, p).unwrap_or(());
            }
            println!("Scanned {:?} tags", db.len());
            db
        }
        Err(_) => TagHintDB::new()
    }
}
