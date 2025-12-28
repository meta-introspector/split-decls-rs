use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn to_compile_error (error : syn :: Error , dummy : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let compile_errors = error . to_compile_error () ; quote :: quote ! (# dummy # compile_errors) }