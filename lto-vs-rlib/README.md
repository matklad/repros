```
13:52:09|~/projects/repros/lto-vs-rlib|master⚡?
λ cargo build -v --release --target wasm32-unknown-unknown
   Compiling lto-vs-rlib v0.1.0 (/home/matklad/projects/repros/lto-vs-rlib)
     Running `rustc --crate-name lto_vs_rlib --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type cdylib --crate-type rlib --emit=dep-info,link -C opt-level=3 -C metadata=007ebacb4e1f5731 --out-dir /home/matklad/projects/repros/lto-vs-rlib/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/home/matklad/projects/repros/lto-vs-rlib/target/wasm32-unknown-unknown/release/deps -L dependency=/home/matklad/projects/repros/lto-vs-rlib/target/release/deps`
    Finished release [optimized] target(s) in 0.39s

13:52:14|~/projects/repros/lto-vs-rlib|master⚡?
λ exa -l target/wasm32-unknown-unknown/release/
drwxr-xr-x    - matklad  9 Jul 13:52 build
drwxr-xr-x    - matklad  9 Jul 13:52 deps
drwxr-xr-x    - matklad  9 Jul 13:52 examples
drwxr-xr-x    - matklad  9 Jul 13:52 incremental
.rw-r--r--  154 matklad  9 Jul 13:52 liblto_vs_rlib.d
.rw-r--r-- 3.7k matklad  9 Jul 13:52 liblto_vs_rlib.rlib
.rw-r--r--  151 matklad  9 Jul 13:52 lto_vs_rlib.d
.rwxr-xr-x 1.5M matklad  9 Jul 13:52 lto_vs_rlib.wasm
```

Now, remove the `rlib` crate type:

```
13:52:53|~/projects/repros/lto-vs-rlib|master⚡?
λ rm -rf target/

13:53:10|~/projects/repros/lto-vs-rlib|master⚡?
λ cargo build -v --release --target wasm32-unknown-unknown
   Compiling lto-vs-rlib v0.1.0 (/home/matklad/projects/repros/lto-vs-rlib)
     Running `rustc --crate-name lto_vs_rlib --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type cdylib --emit=dep-info,link -C opt-level=3 -C lto=fat -C metadata=f24c498f2d854a9d --out-dir /home/matklad/projects/repros/lto-vs-rlib/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/home/matklad/projects/repros/lto-vs-rlib/target/wasm32-unknown-unknown/release/deps -L dependency=/home/matklad/projects/repros/lto-vs-rlib/target/release/deps`
    Finished release [optimized] target(s) in 0.49s

13:53:20|~/projects/repros/lto-vs-rlib|master⚡?
λ exa -l target/wasm32-unknown-unknown/release/
drwxr-xr-x   - matklad  9 Jul 13:53 build
drwxr-xr-x   - matklad  9 Jul 13:53 deps
drwxr-xr-x   - matklad  9 Jul 13:53 examples
drwxr-xr-x   - matklad  9 Jul 13:53 incremental
.rw-r--r-- 151 matklad  9 Jul 13:53 lto_vs_rlib.d
.rwxr-xr-x 209 matklad  9 Jul 13:53 lto_vs_rlib.wasm
```

Note how `crate-type = ["rlib"]` prevents lto from firing, which makes the resulting `.wasm` file far larger.

The root cause here seems to be that `rustc` can't apply `lto` when both rlib and cdylib are specified: https://github.com/rust-lang/rust/issues/86997
