use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < TokenStream > for TokenStream { fn eq (& self , other : & TokenStream) -> bool { self . iter () . eq (other . iter ()) } }
}