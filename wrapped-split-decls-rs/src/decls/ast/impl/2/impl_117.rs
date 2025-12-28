use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl GenBlockKind { pub fn modifier (& self) -> & 'static str { match self { GenBlockKind :: Async => "async" , GenBlockKind :: Gen => "gen" , GenBlockKind :: AsyncGen => "async gen" , } } }