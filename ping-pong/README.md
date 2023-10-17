```
λ cargo run --example reproduce
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/reproduce`
Building rust:
$ cargo build --quiet --release --bin ping-pong

Building Go:
$ go build main.go

Running Rust:
$ ./target/release/ping-pong
360.852151ms
$ taskset --cpu-list 1 ./target/release/ping-pong
4.952019978s

Running Go:
$ ./main
296.946199ms
$ taskset --cpu-list 1 ./main
204.286556ms
```
