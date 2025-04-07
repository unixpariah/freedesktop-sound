use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

pub(crate) static CACHE: LazyLock<Cache> = LazyLock::new(Cache::default);
type SoundMap = BTreeMap<Box<str>, CacheEntry>;
type ThemeMap = BTreeMap<Box<str>, SoundMap>;

#[derive(Default)]
pub(crate) struct Cache(Mutex<ThemeMap>);

#[derive(Debug, Clone, PartialEq)]
pub enum CacheEntry {
    NotFound,
    Found(PathBuf),
    Unknown,
}

impl Cache {
    pub fn insert<P: AsRef<Path>>(&self, theme: &str, sound_name: &str, sound_path: &Option<P>) {
        let mut theme_map = self.0.lock().unwrap();
        let entry = sound_path
            .as_ref()
            .map(|path| CacheEntry::Found(path.as_ref().to_path_buf()))
            .unwrap_or(CacheEntry::NotFound);

        match theme_map.get_mut(theme) {
            Some(sound_map) => {
                sound_map.insert(sound_name.into(), entry);
            }
            None => {
                let mut sound_map = BTreeMap::new();
                sound_map.insert(sound_name.into(), entry);
                theme_map.insert(theme.into(), sound_map);
            }
        }
    }

    pub fn get(&self, theme: &str, sound_name: &str) -> CacheEntry {
        let theme_map = self.0.lock().unwrap();

        theme_map
            .get(theme)
            .map(|sound_map| sound_map.get(sound_name))
            .and_then(|path| path.cloned())
            .unwrap_or(CacheEntry::Unknown)
    }
}
