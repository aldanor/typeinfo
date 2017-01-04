#[macro_use]
extern crate typeinfo;

use std::mem;

use typeinfo::Type::*;
use typeinfo::{Type, TypeInfo, NamedField};

#[test]
fn test_scalar_types() {
    fn check_scalar_type<T: TypeInfo>(ty: Type) {
        assert_eq!(<T as TypeInfo>::type_info(), ty);
        assert_eq!(ty.size(), mem::size_of::<T>());
        assert!(ty.is_scalar() && !ty.is_array() && !ty.is_compound());
    }

    check_scalar_type::<i8>(Int8);
    check_scalar_type::<i16>(Int16);
    check_scalar_type::<i32>(Int32);
    check_scalar_type::<i64>(Int64);
    check_scalar_type::<u8>(UInt8);
    check_scalar_type::<u16>(UInt16);
    check_scalar_type::<u32>(UInt32);
    check_scalar_type::<u64>(UInt64);
    check_scalar_type::<f32>(Float32);
    check_scalar_type::<f64>(Float64);
    check_scalar_type::<bool>(Bool);
    check_scalar_type::<char>(Char);

    if mem::size_of::<usize>() == 4 {
        check_scalar_type::<isize>(Int32);
        check_scalar_type::<usize>(UInt32);
    } else {
        check_scalar_type::<isize>(Int64);
        check_scalar_type::<usize>(UInt64);
    }
}

#[test]
fn test_array_types() {
    let ty = <[u16; 42] as TypeInfo>::type_info();
    assert_eq!(ty, Array(Box::new(UInt16), 42));
    assert_eq!(ty.size(), 2 * 42);
    assert!(ty.is_array() && !ty.is_scalar() && !ty.is_compound());

    let ty = <[[i8; 2]; 3] as TypeInfo>::type_info();
    assert_eq!(ty, Array(Box::new(Array(Box::new(Int8), 2)), 3));
    assert_eq!(ty.size(), 1 * 2 * 3);
    assert!(ty.is_array() && !ty.is_scalar() && !ty.is_compound());
}

#[test]
fn test_tuple_types() {
    let ty = <(i8, u16) as TypeInfo>::type_info();
    let size = mem::size_of::<(i8, u16)>();
    assert_eq!(ty, Tuple(vec![Int8, UInt16], size));
    assert_eq!(ty.size(), size);
    assert!(ty.is_tuple());
}

#[test]
fn test_compound_types() {
    def![#[derive(Clone, Copy)] struct X { a: i32, }];
    let ty = X::type_info();
    assert_eq!(ty, Compound(vec![
        NamedField::new(&Int32, "a", 0)
    ], mem::size_of::<X>()));
    assert_eq!(ty.size(), mem::size_of::<X>());
    assert!(ty.is_compound() && !ty.is_scalar() && !ty.is_array());

    def![#[derive(Clone, Copy)] struct Y { a: u64, x: [X; 2] }];
    let ty = Y::type_info();
    assert_eq!(ty, Compound(vec![
        NamedField::new(&UInt64, "a", 0),
        NamedField::new(&Array(Box::new(X::type_info()), 2), "x", 8),
    ], mem::size_of::<Y>()));
    assert_eq!(ty.size(), mem::size_of::<Y>());
    assert!(ty.is_compound() && !ty.is_scalar() && !ty.is_array());

    def![#[derive(Clone, Copy)] struct Z];
    let ty = Z::type_info();
    assert_eq!(ty, Compound(vec![], mem::size_of::<Z>()));
    assert_eq!(ty.size(), mem::size_of::<Z>());
    assert!(ty.is_compound() && !ty.is_scalar() && !ty.is_array());
}

#[test]
fn test_compound_copy_clone() {
    def![#[derive(Clone, Copy)] struct X { a: char }];
    let x = X { a: '0' };
    let y = x;
    assert_eq!(x.a, y.a);
    assert_eq!(x.clone().a, y.clone().a);
}

#[test]
fn test_struct_attributes() {
    def![#[derive(Clone, Copy)] struct X { a: i8, b: u64 }];
    def![#[repr(packed)] #[derive(Clone, Copy)] struct Y { a: i8, b: u64 }];
    assert!(X::type_info().size() > Y::type_info().size());
}

#[cfg(test)]
mod module {
    def! {
        #[derive(Clone, Copy)] pub struct A {
            x: i32,
            y: i32
        }
    }
    def! {
        #[derive(Clone, Copy)] pub struct B {
            pub x: i32,
            pub y: i32
        }
    }

    def! {
        #[derive(Clone, Copy)] pub struct U;
    }

    pub mod multiple {
        def! {
            #[derive(Clone, Copy)] struct C { x: i32 }
            #[derive(Clone, Copy)] struct D { x: i32 }
        }
        def! {
            #[derive(Clone, Copy)] pub struct E { x: i32 }
            #[derive(Clone, Copy)] pub struct F { x: i32 }
        }
        def! {
            #[derive(Clone, Copy)] pub struct G { pub x: i32 }
            #[derive(Clone, Copy)] pub struct H { pub x: i32 }
        }
        def! {
            #[derive(Clone, Copy)] struct U1;
            #[derive(Clone, Copy)] struct U2
        }
        def! {
            #[derive(Clone, Copy)] pub struct W1;
            #[derive(Clone, Copy)] pub struct W2;
        }
    }
}

#[test]
#[allow(unused_variables, unused_imports)]
fn test_pub_structs_fields() {
    use module::{A, B, U};
    use module::multiple::{E, F, G, H, W1, W2};
    let b = B { x: 1, y: 2 };
}
