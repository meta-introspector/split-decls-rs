use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_attribute] pub fn concretize (_attrs : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { input }