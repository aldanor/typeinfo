#![feature(proc_macro, proc_macro_lib)]

// Because `quote!`.
#![recursion_limit = "192"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate typeinfo;

use proc_macro::TokenStream;
use syn::{Body, VariantData};

#[proc_macro_derive(TypeInfo)]
pub fn type_info(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let name = &ast.ident;
    let (impl_gen, ty_gen, where_clause) = ast.generics.split_for_impl();
    let body = type_info_impl(&ast.body);
    let gen = quote! {
        #[allow(dead_code, unused_variables)]
        impl #impl_gen ::typeinfo::TypeInfo for #name #ty_gen #where_clause {
            fn type_info() -> ::typeinfo::Type {
                let size = ::std::mem::size_of::<#name>();
                let base = 0usize as *const #name;
                #body
            }
        }
    };
    gen.parse().unwrap()
}

fn type_info_impl(body: &Body) -> quote::Tokens {
    match *body {
        Body::Struct(VariantData::Unit) => {
            quote! { ::typeinfo::Type::Compound(vec![], size) }
        },
        Body::Struct(VariantData::Struct(ref fs)) => {
            let name1 = fs.iter().map(|f| &f.ident);
            let name2 = fs.iter().map(|f| &f.ident);
            let ty1 = fs.iter().map(|f| &f.ty);
            let ty2 = fs.iter().map(|f| &f.ty);
            quote! {
                ::typeinfo::Type::Compound(vec![
                    #(::typeinfo::Field::new(
                        &<#ty1 as ::typeinfo::TypeInfo>::type_info(),
                        stringify!(#name1),
                        unsafe { &((*base).#name2) as *const #ty2 as usize }
                    )),*], size)
            }
        },
        Body::Struct(VariantData::Tuple(_)) => {
            unimplemented!()
        },
        Body::Enum(_) => {
            unimplemented!()
        },
    }
}
