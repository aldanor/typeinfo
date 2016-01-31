#[macro_use]
extern crate pod_typeinfo;

def! {
    struct Foo; //~ ERROR no rules expected the token `;`
}
