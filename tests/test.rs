#[macro_use]
extern crate pod_typeinfo;

use std::mem;

use pod_typeinfo::Type::*;
use pod_typeinfo::{Type, TypeInfo, Field};

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
    check_scalar_type::<isize>(ISize);
    check_scalar_type::<usize>(USize);
    check_scalar_type::<f32>(Float32);
    check_scalar_type::<f64>(Float64);
    check_scalar_type::<bool>(Bool);
    check_scalar_type::<char>(Char);
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
fn test_compound_types() {
    def![struct X { a: i32, }];
    let ty = X::type_info();
    assert_eq!(ty, Compound(vec![
        Field::new(&Int32, "a", 0)
    ], mem::size_of::<X>()));
    assert_eq!(ty.size(), mem::size_of::<X>());

    def![struct Y { a: u64, x: [X; 2] }];
    let ty = Y::type_info();
    assert_eq!(ty, Compound(vec![
        Field::new(&UInt64, "a", 0),
        Field::new(&Array(Box::new(X::type_info()), 2), "x", 8),
    ], mem::size_of::<Y>()));
    assert_eq!(ty.size(), mem::size_of::<Y>());
}

#[test]
fn test_compound_copy_clone() {
    def![struct X { a: char }];
    let x = X { a: '0' };
    let y = x;
    assert_eq!(x.a, y.a);
    assert_eq!(x.clone().a, y.clone().a);
}

#[test]
fn test_struct_attributes() {
    def![struct X { a: i8, b: u64 }];
    def![#[repr(packed)] struct Y { a: i8, b: u64 }];
    assert!(X::type_info().size() > Y::type_info().size());
}

#[cfg(test)]
mod module {
    def! {
        pub struct A {
            x: i32,
            y: i32
        }
    }

    def! {
        pub struct B {
            pub x: i32,
            pub y: i32
        }
    }
}

#[test]
#[allow(unused_variables, unused_imports)]
fn test_pub_structs_fields() {
    use module::{A, B};
    let b = B { x: 1, y: 2 };
}
