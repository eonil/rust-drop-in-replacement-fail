
rust-drop-in-replacement-fail
=============================


I am trying to make a drop-in replacement tool for `rustc`.
I followed https://github.com/nrc/stupid-stats at first and failed.
Now I am trying to start from the simplest example, but it also failed.

See `fail1` for source code of drop-in replacement tool.
`foo1` is example project to process with the tool.


Problem
-------
I am using Rust nightly compiler via `rustup`.

    rustc 1.28.0-nightly (a1d4a9503 2018-05-20)
    cargo 1.28.0-nightly (f352115d5 2018-05-15)

I confirmed that `foo1` project gets built with no problem with nightly `cargo`.

    Eonil$ cargo build
        Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    [~/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1] (master)

My tool `fail1` is very simple.

    #![feature(rustc_private)]
    #![feature(link_args)]

    extern crate rustc_driver;

    fn main() {
        rustc_driver::set_sigpipe_handler();
        rustc_driver::main();
    }

I built and install it with `cargo install`.

    Eonil$ cargo clean; cargo install --force
    warning: To build the current package use `cargo build`, to install the current package run `cargo install --path .`
      Installing fail1 v0.1.0 (file:///Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/fail1)
       Compiling fail1 v0.1.0 (file:///Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/fail1)
        Finished release [optimized] target(s) in 0.82s
       Replacing /Users/Eonil/.cargo/bin/fail1
    [~/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/fail1] (master)
    
And I overriden `rustc` with `fail1`.

    export RUSTC=fail1

And moved to `foo1` and build it. It fails.

    Eonil$ cargo clean; cargo build --verbose

       Compiling foo1 v0.1.0 (file:///Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1)
         Running `fail1 --crate-name foo1 src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=733285973e79a539 -C extra-filename=-733285973e79a539 --out-dir /Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/deps -C incremental=/Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/incremental -L dependency=/Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/deps`
    error[E0463]: can't find crate for `std`

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0463`.
    error: Could not compile `foo1`.

    Caused by:
      process didn't exit successfully: `fail1 --crate-name foo1 src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=733285973e79a539 -C extra-filename=-733285973e79a539 --out-dir /Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/deps -C incremental=/Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/incremental -L dependency=/Users/Eonil/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1/target/debug/deps` (exit code: 101)
    [~/Workshop/Playground/bug-reproduction/rust-drop-in-replacement-fail/foo1] (master)

No idea why.


