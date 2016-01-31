#[derive(Clone, Debug)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    ISize,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    USize,
    Float32,
    Float64,
    Char,
    Bool,
    Array(Box<Type>, usize),
    Compound(Vec<Field>, usize)
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            &Type::Int8 | &Type::UInt8 | &Type::Bool => 1,
            &Type::Int16 | &Type::UInt16 => 2,
            &Type::Int32 | &Type::UInt32 | &Type::Float32 => 4,
            &Type::Int64 | &Type::UInt64 | &Type::Float64 => 8,
            &Type::Char => ::std::mem::size_of::<char>(),
            &Type::USize => ::std::mem::size_of::<usize>(),
            &Type::ISize => ::std::mem::size_of::<isize>(),
            &Type::Array(ref ty, num) => ty.size() * num,
            &Type::Compound(_, size) => size,
        }
    }

    pub fn is_array(&self) -> bool {
        if let &Type::Array(_, _) = self { true } else { false }
    }

    pub fn is_compound(&self) -> bool {
        if let &Type::Compound(_, _) = self { true } else { false }
    }

    pub fn is_scalar(&self) -> bool {
        !self.is_array() && !self.is_compound()
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub ty: Type,
    pub name: String,
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

pub trait TypeInfo : Copy {
    fn type_info() -> Type;
}

macro_rules! impl_atomic {
    ($t:ty, $i:ident) => (
        impl $crate::TypeInfo for $t {
            #[inline(always)]
            fn type_info() -> $crate::Type {
                $crate::Type::$i
            }
        }
    )
}

impl_atomic!(i8, Int8);
impl_atomic!(i16, Int16);
impl_atomic!(i32, Int32);
impl_atomic!(i64, Int64);
impl_atomic!(isize, ISize);
impl_atomic!(u8, UInt8);
impl_atomic!(u16, UInt16);
impl_atomic!(u32, UInt32);
impl_atomic!(u64, UInt64);
impl_atomic!(usize, USize);
impl_atomic!(f32, Float32);
impl_atomic!(f64, Float64);
impl_atomic!(char, Char);
impl_atomic!(bool, Bool);

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
    ($($(#[$attr:meta])* struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            struct $s { $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

    ($($(#[$attr:meta])* pub struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            pub struct $s { $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

    ($($(#[$attr:meta])* pub struct $s:ident { $(pub $i:ident: $t:ty),+$(,)* })*) => (
        $(
            #[allow(dead_code)]
            #[derive(Clone, Copy)]
            $(#[$attr])*
            pub struct $s { pub $($i: $t),* }
            def!(@impl $s { $($i: $t),+ } );
        )*
    );

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
