use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FnRetTy { pub fn span (& self) -> Span { match self { & FnRetTy :: Default (span) => span , FnRetTy :: Ty (ty) => ty . span , } } }