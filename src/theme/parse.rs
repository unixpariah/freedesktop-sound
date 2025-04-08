use super::Theme;
use std::path::{Path, PathBuf};

fn sound_theme_section(file: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    ini_core::Parser::new(file)
        .skip_while(|item| *item != ini_core::Item::Section("Sound Theme"))
        .take_while(|item| match item {
            ini_core::Item::Section(value) => *value == "Sound Theme",
            _ => true,
        })
        .filter_map(|item| {
            if let ini_core::Item::Property(key, value) = item {
                Some((key, value?))
            } else {
                None
            }
        })
}

#[derive(Debug)]
pub struct Directory<'a> {
    pub name: &'a str,
}

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

    pub fn inherits<'a>(&self, file: &'a str) -> Vec<&'a str> {
        sound_theme_section(file)
            .find(|&(key, _)| key == "Inherits")
            .map(|(_, parents)| {
                parents
                    .split(',')
                    .filter(|parent| parent != &"freedesktop")
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub(super) fn try_build_sound_path<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    try_build_disabled(name, path.as_ref())
        .or_else(|| try_build_oga(name, path.as_ref()))
        .or_else(|| try_build_ogg(name, path.as_ref()))
        .or_else(|| try_build_wav(name, path.as_ref()))
}

fn try_build_oga<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let oga = path.join(format!("{name}.oga"));

    if oga.exists() { Some(oga) } else { None }
}

fn try_build_ogg<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let ogg = path.join(format!("{name}.ogg"));

    if ogg.exists() { Some(ogg) } else { None }
}

fn try_build_disabled<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let disabled = path.join(format!("{name}.disabled"));

    if disabled.exists() {
        Some(disabled)
    } else {
        None
    }
}

fn try_build_wav<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let wav = path.join(format!("{name}.wav"));

    if wav.exists() { Some(wav) } else { None }
}
