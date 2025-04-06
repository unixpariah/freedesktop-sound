mod theme;

use std::{io::BufRead, path::PathBuf};
use theme::THEMES;

pub fn list_themes() -> Vec<String> {
    let mut themes = THEMES
        .values()
        .flatten()
        .map(|path| &path.index)
        .filter_map(|index| {
            let file = std::fs::File::open(index).ok()?;
            let mut reader = std::io::BufReader::new(file);

            let mut line = String::new();
            while let Ok(read) = reader.read_line(&mut line) {
                if read == 0 {
                    break;
                }

                if let Some(name) = line.strip_prefix("Name=") {
                    return Some(name.trim().to_owned());
                }

                line.clear();
            }

            None
        })
        .collect::<Vec<_>>();
    themes.dedup();
    themes
}

pub struct LookupBuilder<'a> {
    name: &'a str,
    theme: &'a str,
}

impl<'a> LookupBuilder<'a> {
    fn new<'b: 'a>(name: &'b str) -> Self {
        Self {
            name,
            theme: "freedesktop",
        }
    }

    pub fn with_theme<'b: 'a>(mut self, theme: &'b str) -> Self {
        self.theme = theme;
        self
    }

    pub fn find(self) -> Option<PathBuf> {
        self.lookup_in_theme()
    }

    fn lookup_in_theme(&self) -> Option<PathBuf> {
        THEMES
            .get(self.theme)
            .or_else(|| THEMES.get("freedesktop"))
            .and_then(|sound_themes| {
                sound_themes
                    .iter()
                    .find_map(|theme| theme.try_get_sound(self.name))
            })
    }
}

pub fn lookup(name: &str) -> LookupBuilder {
    LookupBuilder::new(name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::lookup;

    #[test]
    fn simple_lookup() {
        let bell = lookup("bell").find();
        assert!(bell
            .is_some_and(|b| b == PathBuf::from("/usr/share/sounds/freedesktop/stereo/bell.oga")));
    }
}
