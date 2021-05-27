```
12:36:27|~/projects/repros/resolver2|HEAD⚡?
λ cargo b -p a --features b/feat-b
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s

12:38:45|~/projects/repros/resolver2|HEAD⚡?
λ cargo b -p a --features c/feat-c
error: none of the selected packages contains these features: c/feat-c

12:38:49|~/projects/repros/resolver2|HEAD⚡?
λ cargo b -p b --features c/feat-c
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

It is surprising that the second command fails.
I'd expect all three commands to work or all there commands to not work.
