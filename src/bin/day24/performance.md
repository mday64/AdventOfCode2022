# Improvements
To move up, down, or stay in place, we have to iterate over blizzards in the
same column (the current column).  Would it be faster to iterate over them
once, and just compare against the 3 different Y values?  Likewise, left,
right, and center, all iterate over the same row, and 3 different X values.

Should I have 5 different booleans for the 5 possible places to move?

# Performance

Cargo.toml
[profile.release]
lto = true
codegen-units = 1

.cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=native"]

    let now = std::time::Instant::now();
    let duration = now.elapsed();
    println!("Part 1: {steps1} in {duration:?}");
    println!("Part 2: {result2} in {duration:?}");

857ba0964e286ae9dbb6a82e807d788ac506aab9:
Part 1: 299 in 5.32225419s
[src/bin/day24/main.rs:195] steps = 299
[src/bin/day24/main.rs:199] steps = 321
[src/bin/day24/main.rs:203] steps = 279
Part 2: 899 in 18.181735106s

499b521d2d3f0e0d9e8db2df5500ad52dd914ca6:
Part 1: 299 in 5.658097003s
[src/bin/day24/main.rs:196] steps = 299
[src/bin/day24/main.rs:200] steps = 321
[src/bin/day24/main.rs:204] steps = 279
Part 2: 899 in 18.236479892s

e681ec8bd1230d80fd5a981fb6c67eb18475a2be:
Part 1: 299 in 5.278383274s
[src/bin/day24/main.rs:184] steps = 299
[src/bin/day24/main.rs:188] steps = 321
[src/bin/day24/main.rs:192] steps = 279
Part 2: 899 in 18.936754685s

ebef3b449a185cff9cd593eb5cc6fc3a46f6aee4:
Part 1: 299 in 5.409689694s
[src/bin/day24/main.rs:184] steps = 299
[src/bin/day24/main.rs:188] steps = 321
[src/bin/day24/main.rs:192] steps = 279
Part 2: 899 in 18.918070339s

6b3c2d1b5381228dfcbbc40fae8c7c974828253a:
Part 1: 299 in 51.349902ms
[src/bin/day24/main.rs:162] steps = 299
[src/bin/day24/main.rs:166] steps = 321
[src/bin/day24/main.rs:170] steps = 279
Part 2: 899 in 18.915084236s

03507e4d835df386ec4b3b9c214ef9a1a93e5532:
Part 1: 299 in 118.162646ms
Part 2: 899 in 283.599166ms

beafbca5885b0554738c379895bac8abd890ec9f:
Part 1: 299 in 106.815187ms
Part 2: 899 in 263.015089ms

807b0f5d3ed74ce11a0ed52d8d94827556b76aba:
Part 1: 299 in 53.026823ms
Part 2: 899 in 127.436554ms

Faster, but messier:
Part 1: 299 in 37.717744ms
Part 2: 899 in 95.898257ms
