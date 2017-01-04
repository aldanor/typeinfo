# typeinfo

[![Build Status](https://travis-ci.org/aldanor/typeinfo.svg?branch=master)](https://travis-ci.org/aldanor/typeinfo)
[![Build Status](https://ci.appveyor.com/api/projects/status/uh34kafh5qs458ue/branch/master?svg=true)](https://ci.appveyor.com/project/aldanor/typeinfo)

[Documentation](http://ivansmirnov.io/typeinfo)

The `typeinfo` crate provides access to type information for POD (*plain old data*)
types at runtime.

## Examples

Defining reflectable struct types only requires wrapping the struct definition in
the [`def!`](http://ivansmirnov.io/typeinfo/typeinfo/macro.def!.html) macro:

```rust
#[macro_use]
extern crate typeinfo;
use typeinfo::TypeInfo;

def! {
    #[derive(Debug)]
    pub struct Color { r: u16, g: u16, b: u16, }

    #[derive(Debug)]
    #[repr(packed)]
    pub struct Palette {
        monochrome: bool,
        colors: [Color; 16]
    }
}

fn main() {
    println!("{:#?}", Palette::type_info());
}
```

Output (whitespace formatted):

```rust
Compound([
    NamedField { ty: Bool, name: "monochrome", offset: 0 },
    NamedField {
        ty: Array(
                Compound([
                    NamedField { ty: UInt16, name: "r", offset: 0 },
                    NamedField { ty: UInt16, name: "g", offset: 2 },
                    NamedField { ty: UInt16, name: "b", offset: 4 }
                ], 6),
            16),
        name: "colors",
        offset: 1
    }
], 97)
```

## License

`typeinfo` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details.
