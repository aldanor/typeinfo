#[macro_use]
extern crate typeinfo;

mod m {
    def! {
        pub struct Bar {
            a: i32,
        }
    }
}

fn main() {
    let _ = m::Bar { a: 1 }; //~ ERROR field `a` of struct `m::Bar` is private
}
