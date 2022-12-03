# 2022AdventOfCode
Advent of code, written in [Rust](https://www.rust-lang.org/). Going into this challenge, I had no experience writing Rust, but I've found this to be a good way to get familiar with the language.

## Day 1
My main confusion on day one had to do with object ownership in Rust. Specifically, with these lines:
```rust
for line in reader.lines() {
    let lstr = line.unwrap().clone();
```
At first, I wasn't cloning the line, which meant that I couldn't perform certain operations on it later on in the loop body. I'm not sure if there's a better/cleaner way of handling this.

Things got significantly more complicated with part 2, because I needed to keep an array of the largest snack counts. While I was at it, I tried to do it for the `N` largest snackbags. I ended up duplicating my code in the loop to properly handle the last snackbag in the file. It's not pretty, but it works for now.

I still don't know why I need `?` after some function calls. I know it has to do with automatic error-handling, but I'm not familiar enough with Rust to know why.

## Day 2
For day 2, the main discovery was the use of `match()`. This allowed me to essentially perform the same steps for parts 1 and 2, just with different matches between values in the left column and values in the right column.

## Day 3
I'm quite proud of my day 3 implementation. I used bitwise operators to map each priority level to a bit in a `u64` variable. Each line was split into left and right halves. Letters were converted to ASCII characters and then mapped to values in the range `1..52` depending on whether they were uppercase or lowercase. 

For each character that occurred on the left half, I'd set the corresponding bit in my `u64` bitmap. After reading the whole left side, I could see which characters were used at least once or not at all.

Then, when reading the left half, I just checked whether that character had already been entered into the bit map. If it had, I'd found the misplaced priority level, and I could simply add it to the total and break out of the loop.

For part two, I just removed the left-right sides of the bag, and treated each bag as a single entity. I had a separate bitmap for each elf in the group, giving me three total bitmaps. Then, I just used bitwise-and to find which element was common among all elves in the group. 

My main discovery for part 2 was that I could use `trailing_zeros()` on the final bitmap to directly find the priority of the common element:
```rust
score += u64::from(shared_chars.trailing_zeros());
```

## Day 4
...

