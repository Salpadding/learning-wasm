#[macro_use]
mod elements;
mod tests;
use elements::module::Module;
use std::fmt;


type AInt = i32;
type BInt = i32;

trait A {
    fn a(&self);
}

impl A for AInt {
    fn a(&self) {
        println!("aint");
    }
}



fn main() {
    let m = Module::default();
    println!("Hello, world!");
}



