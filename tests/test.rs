#[macro_use]
extern crate typeinfo;

use std::mem;

use typeinfo::Type::*;
use typeinfo::{Type, TypeInfo, Field, NamedField};

#[test]
fn test_scalar_types() {
    fn check_scalar_type<T: TypeInfo>(ty: Type) {
        assert_eq!(<T as TypeInfo>::type_info(), ty);
        assert_eq!(ty.size(), mem::size_of::<T>());
        assert!(ty.is_scalar());
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
    assert!(ty.is_array());

    let ty = <[[i8; 2]; 3] as TypeInfo>::type_info();
    assert_eq!(ty, Array(Box::new(Array(Box::new(Int8), 2)), 3));
    assert_eq!(ty.size(), 1 * 2 * 3);
    assert!(ty.is_array());
}

#[test]
fn test_tuple_types() {
    let ty = <(i8, u32) as TypeInfo>::type_info();
    let size = mem::size_of::<(i8, u32)>();
    assert_eq!(ty, Tuple(vec![Field::new(&Int8, 0), Field::new(&UInt32, 4)], size));
    assert_eq!(ty.size(), size);
    assert!(ty.is_tuple());

    let ty = <() as TypeInfo>::type_info();
    assert_eq!(ty, Tuple(vec![], 0));
    assert_eq!(ty.size(), 0);
    assert!(ty.is_tuple());

    def![#[derive(Clone, Copy)] struct X(i32)];
    let ty = X::type_info();
    let size = mem::size_of::<X>();
    assert_eq!(ty, Tuple(vec![
        Field::new(&Int32, 0)
    ], size));
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
    assert!(ty.is_compound());

    def![#[derive(Clone, Copy)] struct Y { a: u64, x: [X; 2] }];
    let ty = Y::type_info();
    assert_eq!(ty, Compound(vec![
        NamedField::new(&UInt64, "a", 0),
        NamedField::new(&Array(Box::new(X::type_info()), 2), "x", 8),
    ], mem::size_of::<Y>()));
    assert_eq!(ty.size(), mem::size_of::<Y>());
    assert!(ty.is_compound());

    def![#[derive(Clone, Copy)] struct Z];
    let ty = Z::type_info();
    assert_eq!(ty, Compound(vec![], mem::size_of::<Z>()));
    assert_eq!(ty.size(), mem::size_of::<Z>());
    assert!(ty.is_compound());
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

    def! {
        #[derive(Clone, Copy)] pub struct T1(i32, i32);
    }

    def! {
        #[derive(Clone, Copy)] pub struct T2(pub i32, pub i32);
    }

    def! {
        #[derive(Clone, Copy)] pub struct T3();
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
        def! {
            #[derive(Clone, Copy)] struct T4(i32, i32);
            #[derive(Clone, Copy)] struct T5(i32, i32)
        }
        def! {
            #[derive(Clone, Copy)] pub struct T6(i32, i32);
            #[derive(Clone, Copy)] pub struct T7(i32, i32)
        }
        def! {
            #[derive(Clone, Copy)] pub struct T8(pub i32, pub i32);
            #[derive(Clone, Copy)] pub struct T9(pub i32, pub i32);
        }
        def! {
            #[derive(Clone, Copy)] struct T10();
            #[derive(Clone, Copy)] struct T11()
        }
        def! {
            #[derive(Clone, Copy)] pub struct T12();
            #[derive(Clone, Copy)] pub struct T13();
        }
    }
}

#[test]
#[allow(unused_variables, unused_imports)]
fn test_pub_structs_fields() {
    use module::{A, B, U, T1, T2, T3};
    use module::multiple::{E, F, G, H, W1, W2, T6, T7, T8, T9, T12, T13};
    let _ = B { x: 1, y: 2 };
    let _ = T2(1, 2);
}
