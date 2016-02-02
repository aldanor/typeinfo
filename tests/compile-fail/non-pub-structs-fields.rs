#[macro_use]
extern crate typeinfo;

mod m {
    def! {
        struct Foo {
            a: i32,
        }
    }

    def! {
        pub struct Bar {
            a: i32,
        }
    }
}

fn main() {
    use m::{Foo, Bar}; //~ ERROR struct `Foo` is private
    let f = Foo { a: 1 }; //~ ERROR field `a` of struct `m::Foo` is private
    let b = Bar { a: 1 }; //~ ERROR field `a` of struct `m::Bar` is private
}
