use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl GenericArg { pub fn span (& self) -> Span { match self { GenericArg :: Lifetime (lt) => lt . ident . span , GenericArg :: Type (ty) => ty . span , GenericArg :: Const (ct) => ct . value . span , } } }