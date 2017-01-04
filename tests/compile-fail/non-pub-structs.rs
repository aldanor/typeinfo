#[macro_use]
extern crate typeinfo;

mod m {
    def! {
        #[derive(Clone, Copy)] struct Foo {
            a: i32,
        }
    }
}

fn main() {
    use m::Foo; //~ ERROR struct `Foo` is private
}
