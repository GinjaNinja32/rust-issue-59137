extern crate libloading;

use libloading::Library;

fn main() {
    Library::new("libchild.so").unwrap();
}
