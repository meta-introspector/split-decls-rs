use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Trait { fn crate_path (& self , zerocopy_crate : & Path) -> Path { match self { Self :: Sized => { parse_quote ! (# zerocopy_crate :: util :: macro_util :: core_reexport :: marker ::# self) } _ => parse_quote ! (# zerocopy_crate ::# self) , } } }