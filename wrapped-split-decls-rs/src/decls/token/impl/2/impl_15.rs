use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl InvisibleOrigin { # [inline] pub fn skip (& self) -> bool { match self { InvisibleOrigin :: MetaVar (_) => false , InvisibleOrigin :: ProcMacro => true , } } }