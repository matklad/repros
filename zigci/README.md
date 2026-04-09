Abusing Zig's dependency mechanism to instantiate build.zig twice with different options for CI
purposes. Which is a horrible idea you should never actually use, unless!

```
λ zig build ci --summary all

Build Summary: 8/8 steps succeeded; 2/2 tests passed
ci success
└─ WriteFile debug success
   ├─ run true success 872us MaxRSS:1M
   │  └─ run test 1 passed 305ms MaxRSS:2M
   │     └─ compile test Debug native success 754ms MaxRSS:242M
   └─ run true cached
      └─ run test 1 passed 317ms MaxRSS:1M
         └─ compile test ReleaseSafe native success 3s MaxRSS:284M
```
