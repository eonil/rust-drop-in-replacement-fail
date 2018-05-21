

#![feature(rustc_private)]
#![feature(link_args)]

extern crate rustc_driver;

fn main() {
    rustc_driver::set_sigpipe_handler();
    rustc_driver::main();
}

