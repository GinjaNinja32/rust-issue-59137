extern crate libloading;
extern crate shared;

use libloading::Library;

fn main() {
    let lib = Library::new("libchild.so").unwrap();
    println!("lib: {:?}", lib);
}
