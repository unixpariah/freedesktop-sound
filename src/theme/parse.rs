use super::{directories::Directory, Theme};
use std::path::{Path, PathBuf};

impl Theme {
    pub(super) fn get_all_directories<'a>(
        &'a self,
        file: &'a str,
    ) -> impl Iterator<Item = Directory<'a>> + 'a {
        ini_core::Parser::new(file).filter_map(|item| {
            if let ini_core::Item::Section(name) = item {
                if name != "Sound Theme" {
                    Some(Directory { name })
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

pub(super) fn try_build_sound_path<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    try_build_oga(name, path)
}

fn try_build_oga<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let oga = path.join(format!("{name}.oga"));

    if oga.exists() {
        Some(oga)
    } else {
        None
    }
}
