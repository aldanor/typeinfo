#[macro_use]
extern crate typeinfo;

def! {
    struct Foo {
        a: i32,
        pub b: i32, //~ ERROR no rules expected the token `b`
    }
}
