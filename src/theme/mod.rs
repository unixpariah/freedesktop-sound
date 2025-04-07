pub mod error;
mod parse;
mod paths;

use error::ThemeError;
use parse::try_build_sound_path;
use paths::{ThemePath, BASE_PATHS};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

type Result<T> = std::result::Result<T, ThemeError>;

pub static THEMES: LazyLock<BTreeMap<String, Vec<Theme>>> = LazyLock::new(get_all_themes);

#[derive(Debug)]
pub struct Theme {
    pub path: ThemePath,
    pub index: PathBuf,
}

impl Theme {
    pub fn try_get_sound(&self, name: &str) -> Option<PathBuf> {
        let file = std::fs::read_to_string(&self.index).unwrap_or_default();
        self.get_sound(file.as_str(), name)
    }

    fn get_sound(&self, file: &str, name: &str) -> Option<PathBuf> {
        self.get_all_directories(file)
            .map(|dir| dir.name)
            .map(|dir| self.path().join(dir))
            .find_map(|directory| try_build_sound_path(name, directory))
    }

    fn path(&self) -> &PathBuf {
        &self.path.0
    }

    pub(crate) fn from_path<P: AsRef<Path>>(path: P, index: Option<&PathBuf>) -> Option<Self> {
        let path = path.as_ref();

        let has_index = path.join("index.theme").exists() || index.is_some();

        if !has_index || !path.is_dir() {
            return None;
        }

        let path = ThemePath(path.into());

        match (index, path.index()) {
            (Some(index), _) => Some(Theme {
                path,
                index: index.clone(),
            }),
            (None, Ok(index)) => Some(Theme { path, index }),
            _ => None,
        }
    }
}

pub(super) fn get_all_themes() -> BTreeMap<String, Vec<Theme>> {
    let mut sound_themes = BTreeMap::<_, Vec<_>>::new();
    let mut found_indices = BTreeMap::new();
    let mut to_revisit = Vec::new();

    BASE_PATHS.iter().for_each(|theme_base_dir| {
        let dir_iter = match theme_base_dir.read_dir() {
            Ok(dir) => dir,
            Err(why) => {
                tracing::error!(?why, dir = ?theme_base_dir, "unable to read sound theme directory");
                return;
            }
        };

        dir_iter.filter_map(std::io::Result::ok).for_each(|entry| {
            let name = entry.file_name();
            let fallback_index = found_indices.get(&name);
            if let Some(theme) = Theme::from_path(entry.path(), fallback_index) {
                if fallback_index.is_none() {
                    found_indices.insert(name.clone(), theme.index.clone());
                }
                let name = name.to_string_lossy().to_string();
                sound_themes.entry(name).or_default().push(theme);
            } else if entry.path().is_dir() {
                to_revisit.push(entry);
            }
        });
    });

    to_revisit.iter().for_each(|entry| {
        let name = entry.file_name();
        let fallback_index = found_indices.get(&name);
        if let Some(theme) = Theme::from_path(entry.path(), fallback_index) {
            let name = name.to_string_lossy().to_string();
            sound_themes.entry(name).or_default().push(theme);
        }
    });

    sound_themes
}
