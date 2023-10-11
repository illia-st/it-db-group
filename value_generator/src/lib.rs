use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ValueGenerator)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        pub fn get_value_generator() -> Arc<fn(String) -> Result<Rc<dyn CellValue>, String>> {
            let value_generator = |raw_value| {
                let value = #ident::builder()
                    .with_raw_value(raw_value)
                    .build();
                match value {
                    Ok(value) => {
                        let wrapped_value: Rc<dyn CellValue> = Rc::new(value);
                        Ok(wrapped_value)
                    },
                    Err(err) => {
                        Err(err)
                    }
                }
            };
            Arc::new(value_generator)
        }
    };
    output.into()
}