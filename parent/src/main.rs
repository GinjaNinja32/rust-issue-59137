extern crate libloading;
extern crate shared;

use libloading::{Library, Symbol};

fn main() {
    let lib = Library::new("libchild.so").unwrap();
    println!("lib: {:?}", lib);
}
