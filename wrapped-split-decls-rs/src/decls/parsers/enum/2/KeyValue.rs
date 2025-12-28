use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub enum KeyValue { Simple (Ident , LitStr) , Block (Ident , proc_macro2 :: TokenStream) , List (Ident , proc_macro2 :: TokenStream) , InlineTable (Ident , proc_macro2 :: TokenStream) , }