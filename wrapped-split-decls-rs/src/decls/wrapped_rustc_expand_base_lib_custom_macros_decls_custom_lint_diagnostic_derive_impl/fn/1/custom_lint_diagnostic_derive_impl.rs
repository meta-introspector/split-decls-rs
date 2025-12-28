use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn custom_lint_diagnostic_derive_impl (s : Structure) -> TokenStream { TokenStream :: new () }
}