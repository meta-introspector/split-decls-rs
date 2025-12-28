use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: invalid_data");
fn invalid_data () -> Error { Error :: from_hresult (WIN32_ERROR (ERROR_INVALID_DATA) . to_hresult ()) }
}