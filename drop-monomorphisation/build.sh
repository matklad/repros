#!/bin/sh
rustc -C opt-level=3 --crate-type rlib a.rs
rustc -C opt-level=3 --crate-type rlib --extern a=liba.rlib b.rs
rustc -C opt-level=3 --crate-type rlib --extern b=libb.rlib -L dependency=. \
  -Zsymbol-mangling-version=v0 --emit=llvm-ir -Cno-prepopulate-passes -Cpasses=name-anon-globals \
  c.rs
