use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct AttrFormatter < 'a > { attrs : & 'a [Attribute] , async_trait : bool , trait_variant : bool , doc : bool , must_use : bool , }
}