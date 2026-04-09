Abusing Zig's dependency mechanism to instantiate build.zig twice with different options for CI
purposes. Which is a horrible idea you should never actually use, unless!

```
λ cd ci && zig build ci --summary all

Build Summary: 7/7 steps succeeded; 2/2 tests passed
ci success
├─ test success
│  └─ run test 1 passed 260ms MaxRSS:2M
│     └─ compile test Debug native success 751ms MaxRSS:235M
└─ test success
   └─ run test 1 passed 338ms MaxRSS:1M
      └─ compile test ReleaseSafe native success 3s MaxRSS:278M```
```
