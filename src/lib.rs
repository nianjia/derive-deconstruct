extern crate proc_macro;
use {
    syn::{Token, DeriveInput, parse_macro_input},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream,
};

#[proc_macro_derive(Deconstruct)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = match ast.data {
        syn::Data::Struct(ref s) => deconstruct_for_struct(&ast, &s.fields),
        _ => panic!("only works with struct"),
    };
    result.into()
}

fn deconstruct_for_struct(ast: &syn::DeriveInput,fields: &syn::Fields) -> proc_macro2::TokenStream
{
    match *fields {
        syn::Fields::Named(ref fields) => {
            deconstruct_impl(&ast, Some(&fields.named), true)
        },
        syn::Fields::Unit => {
            deconstruct_impl(&ast, None, false)
        },
        syn::Fields::Unnamed(ref fields) => {
            deconstruct_impl(&ast, Some(&fields.unnamed), false)
        },
    }
}

fn deconstruct_impl(ast: &syn::DeriveInput,
            fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
            named: bool) -> proc_macro2::TokenStream
{
    let struct_name = &ast.ident;

    let unit = fields.is_none();
    let empty = Default::default();

    let fields: Vec<_> = fields.unwrap_or(&empty)
        .iter()
        .enumerate()
        .map(|(i, f)| FieldExt::new(f, i, named)).collect();

    let types = fields.iter().map(|f| f.as_type());
    let fields = fields.iter().map(|f| f.as_field());
    let ret = fields.clone();
    let fields = if unit {
        quote!()
    } else if named {
        quote![ { #(#fields),* } ]
    } else {
        quote![ ( #(#fields),* ) ]
    };


    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (deconstruct, doc) = (
        syn::Ident::new("deconstruct", proc_macro2::Span::call_site()),
        format!("Define function to deconstruct `{}`.", struct_name)
    );
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #deconstruct(self) -> (#(#types),*)  {
                let #struct_name #fields = self;
                ( #(#ret),* )
            }
        }
    }
}

struct FieldExt<'a> {
    ty: &'a syn::Type,
    ident: syn::Ident,
}
impl<'a> FieldExt<'a> {
    pub fn new(field: &'a syn::Field, idx: usize, named: bool) -> FieldExt<'a> {
        FieldExt {
            ty: &field.ty,
            ident: if named {
                field.ident.clone().unwrap()
            } else {
                syn::Ident::new(&format!("f{}", idx), proc_macro2::Span::call_site())
            },
        }
    }
    pub fn as_type(&self) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        quote!(#ty)
    }

    pub fn as_field(&self) -> proc_macro2::TokenStream {
        let f_name = &self.ident;
        quote!(#f_name)
    }
}
