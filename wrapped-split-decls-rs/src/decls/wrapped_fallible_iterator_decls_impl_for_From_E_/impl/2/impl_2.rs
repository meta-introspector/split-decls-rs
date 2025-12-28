use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , E > From < E > for FoldStop < T , E > { # [inline] fn from (e : E) -> FoldStop < T , E > { FoldStop :: Err (e) } }
}