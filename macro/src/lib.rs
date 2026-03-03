use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, parse_macro_input, spanned::Spanned};


#[proc_macro_derive(Uncollate)]
pub fn derive_uncollate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return Error::new(input.span(), "Uncollate can only be derived for structs").into_compile_error().into();
    };
    let struct_name = &input.ident;
    let uncollated_name = format_ident!("Uncollated{}", struct_name);
    let field_names: Vec<_> = data.fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let get_muts: Vec<_> = field_names.iter().map(|f| format_ident!("{}_mut", f)).collect();
    let field_types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();
    let uncollated = quote! {
        #[derive(Default)]
        pub struct #uncollated_name {
            #(#field_names: std::vec::Vec<#field_types>),*
        }

        impl #uncollated_name {
            #(pub fn #field_names(&self) -> &Vec<#field_types>{
                &self.#field_names
            })*
            #(pub fn #get_muts(&mut self) -> &mut Vec<#field_types>{
                &mut self.#field_names
            })*
        }

        impl uncollate::Uncollated for #uncollated_name{}

        impl uncollate::Collated<#uncollated_name> for #struct_name {
            fn uncollate_one(self, uncollated: &mut #uncollated_name) {
                #(uncollated.#field_names.push(self.#field_names);)*
            }
        }
    };
    quote! {
        #uncollated
    }.into()
}