use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, parse_macro_input, spanned::Spanned};


/// This macro allows splitting a vector of structs into a struct of vectors over each field.
/// This is accomplished in a few steps.
/// 
/// 1. Creates an `UncollatedT` type which has a field corresponding to each field of `T` with type
///    `Vec<F>` and `F` is the type of field in `T`.
/// 2. Impls `Collated` for type `T`. The `Collated` trait is used to add the fields of a single
///    instance of `T` to corresponding vectors in a provided UncollatedT struct instance.
/// 3. The `Uncollate` trait is then able to be used on any type that impls `IntoIter<Item = T>`
///    which provides the `uncollate` method to get an `UncollatedT`
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
/// let uncol_dogs = dogs.uncollate();
/// 
/// assert_eq!(uncol_dogs.name(), vec!["Bandit".to_string(), "Scout".to_string()]);
/// assert_eq!(uncol_dogs.breed(), vec!["Beagle".to_string(), "Pug".to_string()]);
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
    let uncollated_name = format_ident!("Uncollated{}", struct_name);
    let field_names: Vec<_> = data.fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    // let function_names: Vec<_> = field_names.iter().map(|f| format_ident!("uncoll_{}", f)).collect();
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