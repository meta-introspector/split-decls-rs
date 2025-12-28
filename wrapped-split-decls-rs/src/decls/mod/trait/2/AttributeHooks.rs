use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait AttributeHooks : Sized { fn pre_flat_map_node_collect_attr < F > (_cfg : & F , _attr : & ast :: Attribute) ; fn post_flat_map_node_collect_bang (_output : & mut Self , _add_semicolon : AddSemicolon) ; }