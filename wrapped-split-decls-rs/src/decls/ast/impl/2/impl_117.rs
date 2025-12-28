use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl GenBlockKind { pub fn modifier (& self) -> & 'static str { match self { GenBlockKind :: Async => "async" , GenBlockKind :: Gen => "gen" , GenBlockKind :: AsyncGen => "async gen" , } } }
}