use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(QueryString)]
pub fn derive_query_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    TokenStream::from(quote! {
        impl #ident {
            pub fn to_query_string(&self) -> String {
                let mut encoder = ::query_string::__uses::QuerySerializer::new(String::new());

                let object = match ::query_string::__uses::serde_json_to_value(self).unwrap() {
                    ::query_string::__uses::JsonValue::Object(object) => object,
                    _ => panic!("expected a JSON object"),
                };

                for (key, value) in object {
                    if value == ::query_string::__uses::JsonValue::Null {
                        continue;
                    }

                    encoder.append_pair(&key, &value.to_string());
                }

                encoder.finish()
            }
        }
    })
}
