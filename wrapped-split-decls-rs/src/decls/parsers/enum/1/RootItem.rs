use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub enum RootItem { Section (Ident , proc_macro2 :: TokenStream) , KeyValue (KeyValue) , }