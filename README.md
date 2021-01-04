# Building a self-contained game in Rust in (hopefully) under 8 kilobytes

Michal Strehovsky in his article [Building a self-contained game in C# under 8 kilobytes](https://medium.com/@MStrehovsky/building-a-self-contained-game-in-c-under-8-kilobytes-74c3cf60ea04) goes on a journey taking a simple [Snake game](https://en.wikipedia.org/wiki/Snake_(video_game_genre)) written in C# for an initial bloated size of 65 megabytes to tiny final size of 8 kilobytes.

This is my attempt at getting to the same place using Rust. My assumption coming into this is that the journey should be shorter in Rust due to it's smaller runtime.

## Step 1: A default Rust package (147 KB)

Instead of implementing the entire game then pairing down, I decided to pair down first, then build up from there to the working game.

To see where I was starting from I created a new Rust package by running `cargo new r_small_snake`. Then I built it in release mode with `cargo build --release`. The file size of the binary produced was 147 kilobytes.

This is better than the initial 65 megabytes from C#. It's still larger than the final C# size of 8 kilobytes, and I don't have a game yet. Clearly *some* work is going to have to be done.

## Step 2: Size optimized release profile (116 KB)

There are some options I can set in our release profile to reduce the size of our binary. I added the following to my Cargo.toml file

```
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = 'abort'
```

https://github.com/johnthagen/min-sized-rust#optimize-for-size was my source for these.

This brought the binary size down to 116 kilobytes.

## Step 3: #![no_std] (2 KB)

Time to play the biggest card I had coming into this, dropping Rust's standard library.

I changed my `src/main.rs` file to look like the following:

```
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> i32 {
    main()
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

(I need the mainCRTStartup entry point otherwise the binary won't link on Windows. Other workarounds I've tried cause other issues.)

This brought the binary size down to 2 kilobytes. Success! Now I *just* have to implement the game.

## Step 4: libc (15 KB)

Now I need to implement the game. Most of the logic I can translate from the C# version. The biggest glaring thing that I'm missing is console functionallity.

My first attempt at this was `libc` + `printf`. In addition to being a pain to link on Windows, this increased the executable size to around 15 kilobytes. C *has* a runtime.

## Step 5: winapi (2 KB)

I dropped `libc`. Next I tried linking to Windows system calls. The `winapi` crate works great for this.

A binary that can print to the console came in at 3 kilobytes. Telling the linker to 16 bit align the binary dropped this back down to 2k. (I haven't been able to get this to work automatically using cargo config files as of yet.)

```
cargo rustc --release -- -Clink-args="/ALIGN:16"
```

## Step 6: actually implementing the game (4 KB)

After this I *only* had to implement the game. A few hundred lines of code later my binary has grown to ~3.2KB 16 bit aligned, ~4.5KB not. I have a working game though.

If you just want to play it, `cargo run` should work. Use WASD to control the snake.
