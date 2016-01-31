#[macro_use]
extern crate pod_typeinfo;

use std::mem;

use pod_typeinfo::Type::*;
use pod_typeinfo::{Type, TypeInfo};

#[test]
fn test_scalar_types() {
    fn check_type<T: TypeInfo>(ty: Type) {
        assert_eq!(<T as TypeInfo>::type_info(), ty);
        assert_eq!(ty.size(), mem::size_of::<T>());
    }
    check_type::<i8>(Int8);
    check_type::<i16>(Int16);
    check_type::<i32>(Int32);
    check_type::<i64>(Int64);
    check_type::<u8>(UInt8);
    check_type::<u16>(UInt16);
    check_type::<u32>(UInt32);
    check_type::<u64>(UInt64);
    check_type::<isize>(ISize);
    check_type::<usize>(USize);
    check_type::<f32>(Float32);
    check_type::<f64>(Float64);
    check_type::<bool>(Bool);
    check_type::<char>(Char);

}
