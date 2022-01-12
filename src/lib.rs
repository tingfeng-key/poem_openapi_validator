use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod error;

// trait OaiValidator {
//     fn error_msg(errors: ValidationErrors) -> String;
// }

#[proc_macro_derive(OaiValidation)]
pub fn derive_oaivalidation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl OaiValidation for #name {
            fn error_msg(errors: ValidationErrors) -> String {
                let mut result = String::new();
                for (_field, errors) in errors.errors().iter() {
                    match errors {
                        ValidationErrorsKind::Field(err_vec) => {
                            result = err_vec[0].message.as_ref().unwrap().to_string();
                            break;
                        }
                        ValidationErrorsKind::Struct(_) => {}
                        ValidationErrorsKind::List(_) => {}
                    }
                }
                result
            }
        }
        #[poem::async_trait]
        impl<'a> FromRequest<'a> for #name {
            async fn from_request(request: &'a Request, body: &mut RequestBody) -> Result<#name, Error> {
                let data: Vec<u8> = FromRequest::from_request(request, body).await?;
                let value = if data.is_empty() {
                    Value::Null
                } else {
                    serde_json::from_slice(&data).map_err(|err| ParseJsonError {
                        reason: err.to_string(),
                    })?
                };

                let value: Self = serde_json::from_value(value).map_err(|err| ValidatorError {
                    message: format!("参数错误: {}", err.to_string()),
                })?;
                value.validate().map_err(|err| ValidatorError {
                    message: Self::error_msg(err),
                })?;
                Ok(value)
            }
        }
    };
    let s = proc_macro::TokenStream::from(expanded);
    println!("{:#?}", s);
    s
}