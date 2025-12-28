use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn fs_serial_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; serial_setup (input , config , "fs") }