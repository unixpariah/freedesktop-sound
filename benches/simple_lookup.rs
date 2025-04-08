use criterion::{Criterion, black_box, criterion_group, criterion_main};
use freedesktop_sound::lookup;
use std::path::PathBuf;

fn bench_direct_lookup(c: &mut Criterion) {
    c.bench_function("direct_lookup_bell", |b| {
        b.iter(|| {
            let result = lookup(black_box("bell"))
                .with_theme(black_box("oxygen"))
                .find();

            let path = match PathBuf::from("/etc/NIXOS").exists() {
                true => PathBuf::from("/run/current-system/sw/share/sounds/oxygen/stereo/bell.ogg"),
                false => PathBuf::from("/usr/share/sounds/oxygen/stereo/bell.ogg"),
            };

            assert_eq!(result, Some(path));
        })
    });

    c.bench_function("direct_lookup_bell_cache", |b| {
        b.iter(|| {
            let result = lookup(black_box("bell"))
                .with_theme(black_box("oxygen"))
                .with_cache()
                .find();

            let path = match PathBuf::from("/etc/NIXOS").exists() {
                true => PathBuf::from("/run/current-system/sw/share/sounds/oxygen/stereo/bell.ogg"),
                false => PathBuf::from("/usr/share/sounds/oxygen/stereo/bell.ogg"),
            };

            assert_eq!(result, Some(path));
        })
    });
}

fn bench_theme_fallback(c: &mut Criterion) {
    c.bench_function("theme_fallback_lookup", |b| {
        b.iter(|| {
            let result = lookup(black_box("bell"))
                .with_theme(black_box("nonexistent_theme"))
                .find();
            assert!(result.is_some());
        })
    });
}

fn bench_nonexistent_sound(c: &mut Criterion) {
    c.bench_function("nonexistent_sound_lookup", |b| {
        b.iter(|| {
            let result = lookup(black_box("invalid_sound_name"))
                .with_theme(black_box("oxygen"))
                .find();
            assert!(result.is_none());
        })
    });
}

criterion_group!(
    benches,
    bench_direct_lookup,
    bench_theme_fallback,
    bench_nonexistent_sound
);
criterion_main!(benches);
