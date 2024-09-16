Zig:

```console
$ ./zig/zig version
0.12.0-dev.2139+e025ad7b4

$ /zig/zig build -Dstrip=none && eza -l ./zig-out/bin/hoyten && ./zig-out/bin/hoyten

.rwxr-xr-x 1.8M matklad 11 Jan 11:24 ./zig-out/bin/hoyten

thread 32741 panic: integer overflow
/home/matklad/p/repros/zig-stack-trace/main.zig:4:14: 0x100c274 in foo (hoyten)
    return x * x;
             ^
/home/matklad/p/repros/zig-stack-trace/main.zig:9:15: 0x100bbc8 in bar (hoyten)
    return foo(std.math.maxInt(u32));
              ^
/home/matklad/p/repros/zig-stack-trace/main.zig:13:33: 0x100bbb8 in main (hoyten)
    std.debug.print("{}", .{ bar()});
                                ^
/home/matklad/p/repros/zig-stack-trace/zig/lib/std/start.zig:585:37: 0x100bb93 in posixCallMainAndExit (hoyten)
            const result = root.main() catch |err| {
                                    ^
/home/matklad/p/repros/zig-stack-trace/zig/lib/std/start.zig:253:5: 0x100ba41 in _start (hoyten)
    asm volatile (switch (native_arch) {
    ^
???:?:?: 0x0 in ??? (???)

$ ./zig/zig build -Dstrip=strip && eza -l ./zig-out/bin/hoyten && ./zig-out/bin/hoyten

.rwxr-xr-x 21k matklad 11 Jan 11:37 ./zig-out/bin/hoyten

thread 32907 panic: integer overflow
Unable to dump stack trace: debug info stripped

$ ./zig/zig build -Dstrip=objcopy && eza -l ./zig-out/bin/hoyten && ./zig-out/bin/hoyten

.rwxr-xr-x 225k matklad 11 Jan 11:24 ./zig-out/bin/hoyten

thread 33065 panic: integer overflow
Unwind information for `exe:0x10318a0` was not available, trace may be incomplete

???:?:?: 0x100c274 in ??? (exe)
???:?:?: 0x100bbc8 in ??? (exe)
???:?:?: 0x100bbb8 in ??? (exe)
???:?:?: 0x100bb93 in ??? (exe)
???:?:?: 0x100ba41 in ??? (exe)

$ ./zig/zig build -Dstrip=llvm_objcopy && eza -l ./zig-out/bin/hoyten && ./zig-out/bin/hoyten

.rwxr-xr-x 216k matklad 16 Sep 19:26 ./zig-out/bin/hoyten

thread 9239 panic: integer overflow
Unwind information for `exe:0x102ed56` was not available, trace may be incomplete

???:?:?: 0x100a6e7 in ??? (exe)
???:?:?: 0x100a028 in ??? (exe)
???:?:?: 0x100a018 in ??? (exe)
???:?:?: 0x1009ff9 in ??? (exe)
???:?:?: 0x1009ea1 in ??? (exe)
```

Rust:

```console
$ rustc --version
rustc 1.73.0 (cc66ad468 2023-10-03)

$ rustc -C debuginfo=2 -Coverflow-checks=true -C opt-level=3 ./main.rs && eza -l ./main && ./main

.rwxr-xr-x 4.7M matklad 11 Jan 11:39 ./main

thread 'main' panicked at ./main.rs:3:5:
attempt to multiply with overflow
stack backtrace:
   0: rust_begin_unwind
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/panicking.rs:595:5
   1: core::panicking::panic_fmt
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/panicking.rs:67:14
   2: core::panicking::panic
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/panicking.rs:117:5
   3: main::foo
             at ./main.rs:3:5
   4: main::bar
             at ./main.rs:8:12
   5: main::main
             at ./main.rs:12:20
   6: core::ops::function::FnOnce::call_once
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ rustc -C debuginfo=0 -C strip=symbols -Coverflow-checks=true -C opt-level=3 ./main.rs && eza -l ./main && ./main

.rwxr-xr-x 371k matklad 11 Jan 11:40 ./main

thread 'main' panicked at ./main.rs:3:5:
attempt to multiply with overflow
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ rustc -C debuginfo=0 -C strip=debuginfo -Coverflow-checks=true -C opt-level=3 ./main.rs && eza -l ./main && ./main

.rwxr-xr-x 451k matklad 11 Jan 11:40 ./main

thread 'main' panicked at ./main.rs:3:5:
attempt to multiply with overflow
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: main::foo
   4: main::bar
   5: main::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
