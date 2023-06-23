use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize)]
pub struct TagData {
    name: String,
    tags: Vec<String>,
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
