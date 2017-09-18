#![recursion_limit = "1024"]
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;

mod errors;
mod utils;
mod event;
mod track;
mod midifile;

//pub use errors::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
