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
                    return Some(name.trim().into());
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
    use crate::{list_themes, lookup};
    use std::path::PathBuf;

    // Helper function to determine expected path base
    fn get_base_path() -> PathBuf {
        if PathBuf::from("/etc/NIXOS").exists() {
            PathBuf::from("/nix/var/nix/profiles/default/share/sounds")
        } else {
            PathBuf::from("/usr/share/sounds")
        }
    }

    #[test]
    fn test_default_lookup() {
        let bell = lookup("bell").find();
        let expected_path = get_base_path().join("freedesktop/stereo/bell.oga");
        assert!(bell.is_some_and(|b| b == expected_path));
    }

    #[test]
    fn test_theme_specific_lookup() {
        let bell = lookup("bell").with_theme("oxygen").find();
        let expected_path = get_base_path().join("oxygen/stereo/bell.ogg");
        assert!(bell.is_some_and(|b| b == expected_path));
    }

    #[test]
    fn test_nonexistent_sound() {
        let result = lookup("nonexistent_sound").find();
        assert!(result.is_none());
    }

    // Check if it will fallback to default theme
    #[test]
    fn test_nonexistent_theme() {
        let result = lookup("bell").with_theme("nonexistent_theme").find();
        let expected_path = get_base_path().join("freedesktop/stereo/bell.oga");
        assert!(result.is_some_and(|p| p == expected_path));
    }

    #[test]
    fn test_list_themes() {
        let themes = list_themes();
        assert!(!themes.is_empty());
        assert!(themes.contains(&"Default".to_string()));
        assert!(themes.contains(&"Oxygen".to_string()));
        assert!(themes.contains(&"Deepin".to_string()));
    }
}
