use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn parse_target_arch (arch : & str) -> base_db :: target :: Arch { use base_db :: target :: Arch :: * ; match arch { "wasm32" => Wasm32 , "wasm64" => Wasm64 , _ => Other , } }
}