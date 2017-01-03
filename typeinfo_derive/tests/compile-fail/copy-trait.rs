#![feature(proc_macro)]

extern crate typeinfo;
#[macro_use] extern crate typeinfo_derive;

#[derive(TypeInfo)] //~ ERROR `Foo: std::marker::Copy` is not satisfied
struct Foo;
