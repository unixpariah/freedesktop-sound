mod cache;
mod theme;

use cache::{CACHE, CacheEntry};
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
    cache: bool,
}

impl<'a> LookupBuilder<'a> {
    fn new<'b: 'a>(name: &'b str) -> Self {
        Self {
            name,
            theme: "freedesktop",
            cache: false,
        }
    }

    pub fn with_theme<'b: 'a>(mut self, theme: &'b str) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_cache<'b: 'a>(mut self) -> Self {
        self.cache = true;
        self
    }

    pub fn find(self) -> Option<PathBuf> {
        self.lookup_in_theme()
    }

    fn lookup_in_theme(&self) -> Option<PathBuf> {
        if self.cache {
            if let CacheEntry::Found(sound) = self.cache_lookup(self.theme) {
                return Some(sound);
            }
        }

        THEMES
            .get(self.theme)
            .or_else(|| THEMES.get("freedesktop"))
            .and_then(|sound_themes| {
                let sound = sound_themes
                    .iter()
                    .find_map(|theme| theme.try_get_sound(self.name))
                    .or_else(|| {
                        let mut parents = sound_themes
                            .iter()
                            .flat_map(|t| {
                                let file = std::fs::read_to_string(&t.index).unwrap_or_default();
                                t.inherits(file.as_ref())
                                    .into_iter()
                                    .map(String::from)
                                    .collect::<Vec<String>>()
                            })
                            .collect::<Vec<_>>();
                        parents.dedup();
                        parents.into_iter().find_map(|parent| {
                            THEMES.get(&parent).and_then(|parent| {
                                parent.iter().find_map(|t| t.try_get_sound(self.name))
                            })
                        })
                    })
                    .or_else(|| {
                        THEMES.get("freedesktop").and_then(|sound_themes| {
                            sound_themes
                                .iter()
                                .find_map(|theme| theme.try_get_sound(self.name))
                        })
                    });

                if self.cache {
                    self.store(self.theme, sound)
                } else {
                    sound
                }
            })
    }

    #[inline]
    fn cache_lookup(&self, theme: &str) -> CacheEntry {
        CACHE.get(theme, self.name)
    }

    #[inline]
    fn store(&self, theme: &str, sound: Option<PathBuf>) -> Option<PathBuf> {
        CACHE.insert(theme, self.name, &sound);
        sound
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
        let path = match PathBuf::from("/etc/NIXOS").exists() {
            true => {
                PathBuf::from("/run/current-system/sw/share/sounds/freedesktop/stereo/bell.oga")
            }
            false => PathBuf::from("/usr/share/sounds/freedesktop/stereo/bell.oga"),
        };

        assert!(bell.is_some_and(|b| b == path));
    }
}
