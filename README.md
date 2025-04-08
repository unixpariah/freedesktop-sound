 # freedesktop-sound
![crates.io-badge](https://img.shields.io/crates/v/freedesktop-sound)
[![Build Status](https://github.com/unixpariah/freedesktop-sound/actions/workflows/tests.yml/badge.svg)](https://github.com/unixpariah/freedesktop-sound/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/unixpariah/freedesktop-sound/graph/badge.svg?token=49LRWZ9D1K)](https://codecov.io/gh/unixpariah/freedesktop-sound)
[![docs](https://docs.rs/freedesktop-sound/badge.svg)](https://docs.rs/freedesktop-sound/latest/freedesktop-sound/index.html)

 This crate provides a [freedesktop sound](https://specifications.freedesktop.org/sound-theme-spec/latest/sound_lookup.html) lookup implementation.

 It exposes a single `lookup` function to find sound files based on their `name` and `theme`.

 ## Example

 **Simple lookup:**

 The following snippet gets sound file from the default 'freedesktop' theme.

 ```rust
 use freedesktop_sound::lookup;

 let sound = lookup("bell").find();
```

**Complex lookup**

If you have specific requirements for your lookup you can use the provided builder functions:

```rust
use freedesktop_sound::lookup;

let sound = lookup("bell")
    .with_theme("oxygen")
    .find();
```

 **Cache:**

 If your application is going to repeat the same sound lookups multiple times
 you can use the internal cache to improve performance.

 ```rust
 use freedesktop_sound::lookup;

 let sound = lookup("bell")
     .with_theme("oxygen")
     .with_cache()
     .find();
```

## Running tests

To run tests, it's recommended to use Docker Compose, which offers images for fhs compliant and not compliant distributions:

```
docker compose up
```
