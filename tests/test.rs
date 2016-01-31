#[macro_use]
extern crate pod_typeinfo;

use std::mem;

use pod_typeinfo::Type::*;
use pod_typeinfo::{Type, TypeInfo};

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
