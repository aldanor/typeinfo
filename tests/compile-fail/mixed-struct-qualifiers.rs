#[macro_use]
extern crate typeinfo;

def! {
    struct Foo {
        a: i32,
        b: i32,
    }

    pub struct Bar { //~ ERROR no rules expected the token `pub`
        a: i32,
        b: i32,
    }
}
