use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validation)]
pub fn derive_pov_validation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn is_error(&self) -> Option<String> {
                match self.validate() {
                    Ok(_) => None,
                    Err(ref errors) => {
                        let mut result = String::new();
                        for (_field, errors) in errors.errors().iter() {
                            match errors {
                                validator::ValidationErrorsKind::Field(err_vec) => {
                                    result = err_vec[0].message.as_ref().unwrap().to_string();
                                    break;
                                }
                                validator::ValidationErrorsKind::Struct(_) => {}
                                validator::ValidationErrorsKind::List(_) => {}
                            }
                        }
                        Some(result)
                    },
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}