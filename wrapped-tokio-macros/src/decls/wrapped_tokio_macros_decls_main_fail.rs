use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Always fails with the error message below.
/// ```text
/// The #[tokio::main] macro requires rt or rt-multi-thread.
/// ```
#[proc_macro_attribute]
pub fn main_fail(_args: TokenStream, _item: TokenStream) -> TokenStream {
    syn::Error::new(
            proc_macro2::Span::call_site(),
            "The #[tokio::main] macro requires rt or rt-multi-thread.",
        )
        .to_compile_error()
        .into()
}
