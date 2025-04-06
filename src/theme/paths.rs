use super::error::ThemeError;
use std::{path::PathBuf, sync::LazyLock};
use xdg::BaseDirectories;

pub(crate) static BASE_PATHS: LazyLock<Vec<PathBuf>> = LazyLock::new(sound_theme_base_paths);

fn sound_theme_base_paths() -> Vec<PathBuf> {
    BaseDirectories::new()
        .map(|bd| {
            bd.get_data_dirs()
                .into_iter()
                .map(|p| p.join("sounds"))
                .filter(|p| p.exists())
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone, Debug)]
pub struct ThemePath(pub PathBuf);

impl ThemePath {
    pub(super) fn index(&self) -> super::Result<PathBuf> {
        let index = self.0.join("index.theme");

        if !index.exists() {
            return Err(ThemeError::ThemeIndexNotFound(index));
        }

        Ok(index)
    }
}

#[cfg(test)]
mod test {
    use crate::theme::paths::sound_theme_base_paths;
    use crate::theme::{get_all_themes, Theme};

    #[test]
    fn should_get_all_themes() {
        let themes = get_all_themes();
        assert!(themes.contains_key("freedesktop"));
    }

    #[test]
    fn should_get_theme_paths_ordered() {
        let base_paths = sound_theme_base_paths();
        assert!(!base_paths.is_empty())
    }

    #[test]
    fn should_read_theme_index() {
        let themes = get_all_themes();
        let themes: Vec<&Theme> = themes.values().flatten().collect();
        assert!(!themes.is_empty());
    }
}
