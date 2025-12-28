use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn serial_setup (input : proc_macro2 :: TokenStream , config : Config , prefix : & str ,) -> proc_macro2 :: TokenStream { core_setup (input , & config , prefix , "serial") }
}