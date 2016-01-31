#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "nightly", plugin(clippy))]

/// POD (*plain old data*) type: scalar, fixed-size array or compound (struct).
/// May be arbitrarily nested.
#[derive(Clone, Debug)]
pub enum Type {
    /// 1-byte signed integer
    Int8,
    /// 2-byte signed integer
    Int16,
    /// 4-byte signed integer
    Int32,
    /// 8-byte signed integer
    Int64,
    /// pointer-sized signed integer
    ISize,
    /// 1-byte unsigned integer
    UInt8,
    /// 2-byte unsigned integer
    UInt16,
    /// 3-byte unsigned integer
    UInt32,
    /// 4-byte unsigned integer
    UInt64,
    /// pointer-sized unsigned integer
    USize,
    /// 4-byte floating-point number
    Float32,
    /// 8-byte floating-point number
    Float64,
    /// character type
    Char,
    /// 1-byte boolean type
    Bool,
    /// fixed-size array with POD elements
    Array(Box<Type>, usize),
    /// compound type whose fields are POD
    Compound(Vec<Field>, usize),
}

impl Type {
    /// Returns the total size of a type value in bytes.
    pub fn size(&self) -> usize {
        match *self {
            Type::Int8 | Type::UInt8 | Type::Bool => 1,
            Type::Int16 | Type::UInt16 => 2,
            Type::Int32 | Type::UInt32 | Type::Float32 => 4,
            Type::Int64 | Type::UInt64 | Type::Float64 => 8,
            Type::Char => ::std::mem::size_of::<char>(),
            Type::USize => ::std::mem::size_of::<usize>(),
            Type::ISize => ::std::mem::size_of::<isize>(),
            Type::Array(ref ty, num) => ty.size() * num,
            Type::Compound(_, size) => size,
        }
    }

    /// Returns true if the underlying type is a scalar.
    pub fn is_scalar(&self) -> bool {
        !self.is_array() && !self.is_compound()
    }

    /// Returns true if the underlying type is a fixed-size array.
    pub fn is_array(&self) -> bool {
        if let Type::Array(_, _) = *self { true } else { false }
    }

    /// Returns true if the underlying type is compound.
    pub fn is_compound(&self) -> bool {
        if let Type::Compound(_, _) = *self { true } else { false }
    }
}

/// Field of a compound type: type, name and offset from the beginning of the struct.
#[derive(Clone, Debug)]
pub struct Field {
    /// field value type
    pub ty: Type,
    /// field name
    pub name: String,
    /// offset to the beginning of the struct
    pub offset: usize,
}

impl Field {
    pub fn new<S, U>(ty: &Type, name: S, offset: U) -> Field
    where S: Into<String>, U: Into<usize> {
        Field {
            ty: ty.clone(),
            name: name.into(),
            offset: offset.into()
        }
    }
}

pub trait TypeInfo: Copy {
    /// Returns the runtime type information for the implementing type.
    fn type_info() -> Type;
}

macro_rules! impl_scalar {
    ($t:ty, $i:ident) => (
        impl $crate::TypeInfo for $t {
            #[inline(always)]
            fn type_info() -> $crate::Type {
                $crate::Type::$i
            }
        }
    )
}

// implement TypeInfo for built-in scalar types
impl_scalar!(i8, Int8);
impl_scalar!(i16, Int16);
impl_scalar!(i32, Int32);
impl_scalar!(i64, Int64);
impl_scalar!(isize, ISize);
impl_scalar!(u8, UInt8);
impl_scalar!(u16, UInt16);
impl_scalar!(u32, UInt32);
impl_scalar!(u64, UInt64);
impl_scalar!(usize, USize);
impl_scalar!(f32, Float32);
impl_scalar!(f64, Float64);
impl_scalar!(char, Char);
impl_scalar!(bool, Bool);

macro_rules! impl_array {
    ($($n:expr),*$(,)*) => {
        $(
            impl<T: $crate::TypeInfo> $crate::TypeInfo for [T; $n] {
                #[inline(always)]
                fn type_info() -> $crate::Type {
                    $crate::Type::Array(
                        Box::new(<T as $crate::TypeInfo>::type_info()),
                        $n
                    )
                }
            }
        )*
    };
}

// implement TypeInfo for fixed-size arrays of lengths 0..63
impl_array!(
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
);

#[macro_export]
macro_rules! def {
    // private struct, private fields
    ($($(#[$attr:meta])* struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            struct $s { $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

    // public struct, private fields
    ($($(#[$attr:meta])* pub struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            pub struct $s { $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

    // public struct, public fields
    ($($(#[$attr:meta])* pub struct $s:ident { $(pub $i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            pub struct $s { pub $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

    // implement TypeInfo trait
    (@impl $s:ident { $($i:ident: $t:ty),+ }) => (
        impl $crate::TypeInfo for $s {
            fn type_info() -> $crate::Type {
                let base = 0usize as *const $s;
                $crate::Type::Compound(vec![$(
                    $crate::Field::new(
                        &<$t as $crate::TypeInfo>::type_info(),
                        stringify!($i),
                        unsafe { &((*base).$i) as *const $t as usize}
                    )
                ),*], ::std::mem::size_of::<$s>())
            }
        }
    );
}

#[cfg(test)]
pub mod tests {
    use super::TypeInfo;

    def! {
        #[derive(Debug)]
        pub struct Foo {
            pub a: bool,
            pub b: i32,
        }

        pub struct Bar {
            pub foo: [Foo; 2],
            pub s: f64
        }
    }

    #[test]
    fn test_smoke() {
        println!("{:#?}", Bar::type_info());
    }
}
