use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NfCore)]
pub fn derive_nfcore(input: TokenStream) -> TokenStream {
    // parse the input tokens into a stream
    let input = parse_macro_input!(input as DeriveInput);

    // pull out the name of the struct the user wants to implement NfCore for,
    // and set it to lowercase
    let struct_name = input.ident;
    let struct_name_str = struct_name.to_string();
    let pipeline_name = struct_name_str.to_lowercase();

    // Define a list of allowed pipeline names.
    // TODO: this could be generated or loaded from a file.
    const ALLOWED_PIPELINES: &[&str] = &["viralrecon", "scrnaseq"];

    // Check if the lowercase struct name is in the allowed list.
    if !ALLOWED_PIPELINES.contains(&pipeline_name.as_str()) {
        return syn::Error::new_spanned(
            struct_name,
            format!("There is not an nf-core pipeline named '{pipeline_name}', so the trait `NfCore` cannot be implemented and a samplesheet cannot be generated."),
        )
        .to_compile_error()
        .into();
    }

    // Expand the code back out into the required tokens to implement the `NfCore` trait at compile-time
    let generics = input.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics NfCore for #struct_name #type_generics #where_clause {}
    };

    expanded.into()
}
