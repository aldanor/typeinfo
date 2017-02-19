#![cfg_attr(feature = "unstable", feature(plugin))]
#![cfg_attr(feature = "unstable", plugin(clippy))]

//! The purpose of this crate is to provide an easy way to query the runtime type
//! information (such as field names, offsets and types) for POD (*plain old data*) types,
//! and to allow creating such types without the need for much boilerplate. This information
//! is extremely useful when communicating with foreign low-level libraries, and, more
//! generally, for any kind of serialization/deserialization work.
//!
//! The core functionality is accessible through the
//! [`type_info`](trait.TypeInfo.html#tymethod.type_info) static
//! method of the [`TypeInfo`](trait.TypeInfo.html) trait which
//! comes implemented for all built-in scalar types and fixed-size arrays, and which can
//! be easily implemented for user types by using the [`def!`](macro.def!.html) macro.
//!
//! # Examples
//!
//! Defining reflectable struct types only requires wrapping the struct definition in
//! [`def!`](macro.def!.html):
//!
//! ```ignore
//! #[macro_use]
//! extern crate typeinfo;
//! use typeinfo::TypeInfo;
//!
//! def! {
//!     #[derive(Clone, Copy, Debug)]
//!     pub struct Color { r: u16, g: u16, b: u16, }
//!
//!     #[derive(Clone, Copy, Debug)]
//!     #[repr(packed)]
//!     pub struct Palette {
//!         monochrome: bool,
//!         colors: [Color; 16]
//!     }
//! }
//!
//! fn main() {
//!     println!("{:#?}", Palette::type_info());
//! }
//! ```
//!
//! Output (whitespace formatted):
//!
//! ```ignore
//! Compound([
//!     NamedField { ty: Bool, name: "monochrome", offset: 0 },
//!     NamedField {
//!         ty: Array(
//!                 Compound([
//!                     NamedField { ty: UInt16, name: "r", offset: 0 },
//!                     NamedField { ty: UInt16, name: "g", offset: 2 },
//!                     NamedField { ty: UInt16, name: "b", offset: 4 }
//!                 ], 6),
//!             16),
//!         name: "colors",
//!         offset: 1
//!     }
//! ], 97)
//! ```

/// Represents a POD type: scalar, fixed-size array or compound (struct).
/// May be arbitrarily nested.
#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    /// 1-byte signed integer
    Int8,
    /// 2-byte signed integer
    Int16,
    /// 4-byte signed integer
    Int32,
    /// 8-byte signed integer
    Int64,
    /// 1-byte unsigned integer
    UInt8,
    /// 2-byte unsigned integer
    UInt16,
    /// 3-byte unsigned integer
    UInt32,
    /// 4-byte unsigned integer
    UInt64,
    /// 4-byte floating-point number
    Float32,
    /// 8-byte floating-point number
    Float64,
    /// 4-byte unicode character type
    Char,
    /// 1-byte boolean type
    Bool,
    /// fixed-size array with POD elements
    Array(Box<Type>, usize),
    /// compound type whose fields are POD
    Compound(Vec<NamedField>, usize),
    /// tuple or a tuple struct with POD elements
    Tuple(Vec<Field>, usize),
}

impl Type {
    /// Returns the total size of a type value in bytes.
    pub fn size(&self) -> usize {
        match *self {
            Type::Int8 | Type::UInt8 | Type::Bool => 1,
            Type::Int16 | Type::UInt16 => 2,
            Type::Int32 | Type::UInt32 | Type::Float32 | Type::Char => 4,
            Type::Int64 | Type::UInt64 | Type::Float64 => 8,
            Type::Array(ref ty, num) => ty.size() * num,
            Type::Compound(_, size) |
            Type::Tuple(_, size) => size,
        }
    }

    /// Returns true if the underlying type is a scalar.
    pub fn is_scalar(&self) -> bool {
        !self.is_array() && !self.is_compound() && !self.is_tuple()
    }

    /// Returns true if the underlying type is a fixed-size array.
    pub fn is_array(&self) -> bool {
        if let Type::Array(_, _) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the underlying type is compound.
    pub fn is_compound(&self) -> bool {
        if let Type::Compound(_, _) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the underlying type is a tuple or a tuple struct.
    pub fn is_tuple(&self) -> bool {
        if let Type::Tuple(_, _) = *self {
            true
        } else {
            false
        }
    }
}

/// Named field of a compound type: contains type, name and offset from the origin.
#[derive(Clone, PartialEq, Debug)]
pub struct NamedField {
    /// field value type
    pub ty: Type,
    /// field name
    pub name: String,
    /// offset to the beginning of the struct
    pub offset: usize,
}

impl NamedField {
    pub fn new<S: Into<String>>(ty: &Type, name: S, offset: usize) -> NamedField {
        NamedField {
            ty: ty.clone(),
            name: name.into(),
            offset: offset,
        }
    }
}

/// Anonymous field of a tuple or a tuple struct: contains type and offset from the origin.
#[derive(Clone, PartialEq, Debug)]
pub struct Field {
    /// field value type
    pub ty: Type,
    /// offset to the beginning of the struct
    pub offset: usize,
}

impl Field {
    pub fn new(ty: &Type, offset: usize) -> Field {
        Field {
            ty: ty.clone(),
            offset: offset,
        }
    }
}
/// Trait implemented by copyable POD data types with fixed size, enables
/// runtime reflection.
///
/// This trait is implemented by default for all built-in scalar types (integer,
/// floating-point, boolean and character), and there's a generic implementation
/// for fixed-size arrays. Note that pointer-sized integer types `isize` /
/// `usize` map to either `Int32` / `UInt32` or `Int64` / `UInt64` respectively,
/// depending on the host platform.
///
/// The easiest way to generate an implementation for a compound type is to use
/// the provided [`def!`](macro.def!.html) macro.
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
impl_scalar!(u8, UInt8);
impl_scalar!(u16, UInt16);
impl_scalar!(u32, UInt32);
impl_scalar!(u64, UInt64);
impl_scalar!(f32, Float32);
impl_scalar!(f64, Float64);
impl_scalar!(char, Char);
impl_scalar!(bool, Bool);

#[cfg(target_pointer_width = "32")]
impl_scalar!(isize, Int32);
#[cfg(target_pointer_width = "64")]
impl_scalar!(isize, Int64);

#[cfg(target_pointer_width = "32")]
impl_scalar!(usize, UInt32);
#[cfg(target_pointer_width = "64")]
impl_scalar!(usize, UInt64);

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
impl_array! {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
}

/// Compound type constructor that implements [`TypeInfo`](trait.TypeInfo.html)
/// trait automatically.
///
/// This macro can be used anywhere a normal struct definition can be placed, supports
/// visibility qualifiers, struct attributes, nested datatypes and multiple struct
/// definitions inside one invocation.
///
/// `def!` defines the type as given, derives `Clone` and `Copy`, and implements the
/// [`TypeInfo`](trait.TypeInfo.html) trait so the type information is readily accessible
/// at runtime.
///
/// *Note:* due to certain limitations of the macro system, a single macro invocation may
/// only contain definitions where both fields and structs have the same visibility qualifier.
///
/// # Examples
/// ```ignore
/// def! {
///     #[derive(Clone, Copy)]
///     pub struct Color {
///         r: u8,
///         g: u8,
///         b: u8,
///     }
///
///     #[derive(Clone, Copy)]
///     pub struct Palette {
///         colors: [Color; 16]
///     }
/// }
/// ```
#[macro_export]
macro_rules! def {
    // private unit struct
    ($($(#[$attr:meta])* struct $s:ident);+$(;)*) => (
        $($(#[$attr])* struct $s; def!(@impl_struct $s { });)*
    );

    // public unit struct
    ($($(#[$attr:meta])* pub struct $s:ident);+$(;)*) => (
        $($(#[$attr])* pub struct $s; def!(@impl_struct $s { });)*
    );

    // private struct, private fields
    ($($(#[$attr:meta])* struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $($(#[$attr])* struct $s { $($i: $t),+ } def!(@impl_struct $s { $($i: $t),+ } );)*
    );

    // public struct, private fields
    ($($(#[$attr:meta])* pub struct $s:ident { $($i:ident: $t:ty),+$(,)* })*) => (
        $($(#[$attr])* pub struct $s { $($i: $t),+ } def!(@impl_struct $s { $($i: $t),+ } );)*
    );

    // public struct, public fields
    ($($(#[$attr:meta])* pub struct $s:ident { $(pub $i:ident: $t:ty),+$(,)* })*) => (
        $($(#[$attr])* pub struct $s { $(pub $i: $t),+ } def!(@impl_struct $s { $($i: $t),+ } );)*
    );

    // private tuple struct, private fields
    ($($(#[$attr:meta])* struct $s:ident ($($t:ty),+$(,)*));*$(;)*) => (
        $($(#[$attr])* struct $s ($($t),+); def!(@impl_tuple $s $($t),+);)*
    );

    // public tuple struct, public fields
    ($($(#[$attr:meta])* pub struct $s:ident ($(pub $t:ty),+$(,)*));*$(;)*) => (
        $($(#[$attr])* pub struct $s ($(pub $t),+); def!(@impl_tuple $s $($t),+);)*
    );

    // public tuple struct, private fields
    ($($(#[$attr:meta])* pub struct $s:ident ($($t:ty),+$(,)*));*$(;)*) => (
        $($(#[$attr])* pub struct $s ($($t),+); def!(@impl_tuple $s $($t),+);)*
    );

    // private unit tuple struct
    ($($(#[$attr:meta])* struct $s:ident ());+$(;)*) => (
        $($(#[$attr])* struct $s(); def!(@impl_tuple $s);)*
    );

    // public unit tuple struct
    ($($(#[$attr:meta])* pub struct $s:ident ());+$(;)*) => (
        $($(#[$attr])* pub struct $s(); def!(@impl_tuple $s);)*
    );

    // implement TypeInfo trait for structs
    (@impl_struct $s:ident { $($i:ident: $t:ty),* }) => (
        impl $crate::TypeInfo for $s {
            #[allow(dead_code, unused_variables)]
            fn type_info() -> $crate::Type {
                $crate::Type::Compound(vec![$(
                    $crate::NamedField::new(
                        &<$t as $crate::TypeInfo>::type_info(),
                        stringify!($i),
                        unsafe { &((*(0usize as *const $s)).$i) as *const _ as usize }
                    )
                ),*], ::std::mem::size_of::<$s>())
            }
        }
    );

    (@replace_with $a:tt $b:tt) => ($b);

    (@parse_tuple_fields [$($s:ident)*] $origin:ident $fields:ident | $t:ty $(,$tt:ty)*) => (
        let &$($s)*(.., ref f, $(def!(@replace_with $tt _),)*) = unsafe { &*$origin };
        $fields.push($crate::Field::new(
            &<$t as $crate::TypeInfo>::type_info(),
            f as *const _ as usize));
        def!(@parse_tuple_fields [$($s)*] $origin $fields | $($tt),*);
    );

    (@parse_tuple_fields [$($s:ident)*] $origin:ident $fields:ident |) => ();

    // implement TypeInfo trait for tuple structs
    (@impl_tuple $s:ident $($tt:ty),*) => (
        impl $crate::TypeInfo for $s {
            #[allow(unused_variables, unused_mut)]
            fn type_info() -> $crate::Type {
                let origin = 0usize as *const $s;
                let mut fields = Vec::<$crate::Field>::new();
                def!(@parse_tuple_fields [$s] origin fields | $($tt),*);
                $crate::Type::Tuple(fields, ::std::mem::size_of::<$s>())
            }
        }
    );
}

macro_rules! impl_tuple {
    ($t:ident) => {
        impl<$t> $crate::TypeInfo for ($t,) where $t: $crate::TypeInfo {
            #[inline(always)]
            fn type_info() -> $crate::Type {
                let origin = 0usize as *const ($t,);
                let &(ref f,) = unsafe { &*origin };
                $crate::Type::Tuple(
                    vec![$crate::Field::new(
                        &<$t as $crate::TypeInfo>::type_info(),
                        f as *const _ as usize)],
                    ::std::mem::size_of::<($t,)>())
            }
        }

        impl $crate::TypeInfo for () {
            #[inline(always)]
            fn type_info() -> $crate::Type {
                $crate::Type::Tuple(vec![], ::std::mem::size_of::<()>())
            }
        }
    };

    ($t:ident, $($tt:ident),*) => {
        impl<$t, $($tt),*> $crate::TypeInfo for ($t, $($tt),*)
        where $t: $crate::TypeInfo, $($tt: $crate::TypeInfo),* {
            #[allow(dead_code, unused_variables)]
            fn type_info() -> $crate::Type {
                let origin = 0usize as *const ($t, $($tt),*);
                let mut fields = Vec::<$crate::Field>::new();
                def!(@parse_tuple_fields [] origin fields | $t, $($tt),*);
                $crate::Type::Tuple(fields, ::std::mem::size_of::<($t, $($tt),*)>())
            }
        }

        impl_tuple!($($tt),*);
    };
}

// implement TypeInfo for tuples of sizes 0..15
impl_tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15 }
