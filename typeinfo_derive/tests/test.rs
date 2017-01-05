#![feature(proc_macro)]

extern crate typeinfo;
#[macro_use]
extern crate typeinfo_derive;

use std::mem;

use typeinfo::Type::*;
use typeinfo::{TypeInfo, NamedField};

#[test]
fn test_compound_types() {
    #[derive(Copy, Clone, TypeInfo)]
    struct X {
        a: i32,
    };
    let ty = X::type_info();
    assert_eq!(ty,
               Compound(vec![NamedField::new(&Int32, "a", 0)], mem::size_of::<X>()));
    assert_eq!(ty.size(), mem::size_of::<X>());
    assert!(ty.is_compound());

    #[derive(Copy, Clone, TypeInfo)]
    struct Y {
        a: u64,
        x: [X; 2],
    };
    let ty = Y::type_info();
    assert_eq!(ty,
               Compound(vec![NamedField::new(&UInt64, "a", 0),
                             NamedField::new(&Array(Box::new(X::type_info()), 2), "x", 8)],
                        mem::size_of::<Y>()));
    assert_eq!(ty.size(), mem::size_of::<Y>());
    assert!(ty.is_compound());

    #[derive(Copy, Clone, TypeInfo)]
    struct Z;
    let ty = Z::type_info();
    assert_eq!(ty, Compound(vec![], mem::size_of::<Z>()));
    assert_eq!(ty.size(), mem::size_of::<Z>());
    assert!(ty.is_compound());
}