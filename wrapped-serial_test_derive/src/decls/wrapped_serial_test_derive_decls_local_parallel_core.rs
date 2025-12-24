use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn local_parallel_core(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let config = get_config(attr);
    parallel_setup(input, config, "local")
}
