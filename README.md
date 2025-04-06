 # freedesktop-sound

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

let icon = lookup("bell")
    .with_theme("oxygen")
    .find();
```

## Running tests

Tests require Docker to resolve sound file paths correctly.

```
docker build -t freedesktop-sound .
docker run freedesktop-sound
```
