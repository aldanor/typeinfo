#[macro_use]
extern crate typeinfo;

def! {
    struct Foo; //~ ERROR no rules expected the token `;`
}
