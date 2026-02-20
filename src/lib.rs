use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, parse_macro_input, spanned::Spanned};


/// Derives the `UncollateT` trait from struct `T`.
/// 
/// 
/// For each field `T.field: F` `UncollateT` provides `fn uncoll_field(&self) -> Vec<F>`.
/// `UncollateT` is implemented for objects that are sequences of T aka `impl Deref<Target = [T]>`.
/// 
/// 
///
/// # Examples
///
/// ```
/// use uncollate::derive_uncollate;
///
/// #[derive(Uncollate)]
/// struct Dog {
///     name: String,
///     breed: String
/// }
/// 
/// let dogs = vec![
///     Dog {name: "Bandit".to_string(), breed: "Beagle".to_string()}, 
///     Dog {name: "Scout".to_string(), breed: "Pug".to_string()}
/// ];
/// 
/// assert_eq!(dogs.uncoll_name(), vec!["Bandit".to_string(), "Scout".to_string()]);
/// assert_eq!(dogs.uncoll_breed(), vec!["Beagle".to_string(), "Pug".to_string()]);
/// ```
#[proc_macro_derive(Uncollate)]
pub fn derive_uncollate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return Error::new(input.span(), "Uncollate can only be derived for structs").into_compile_error().into();
    };
    let struct_name = &input.ident;
    let trait_name = format_ident!("Uncollate{}", struct_name);
    let field_names: Vec<_> = data.fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let function_names: Vec<_> = field_names.iter().map(|f| format_ident!("uncoll_{}", f)).collect();
    let field_types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();
    let uncollate = quote! {
        pub trait #trait_name {
            #(fn #function_names(&self) -> std::vec::Vec<#field_types>; )*
        }
    };

    let impls = quote! {
        impl<T: std::ops::Deref<Target = [#struct_name]>> #trait_name for T {
            #(fn #function_names(&self) -> std::vec::Vec<#field_types> {
                self.iter().map(|f| f.#field_names).collect()
            })*
        } 
    };
    quote! {
        #uncollate
        #impls
    }.into()
}