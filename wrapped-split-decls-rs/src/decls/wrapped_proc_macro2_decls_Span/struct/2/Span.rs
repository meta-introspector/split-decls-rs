use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A region of source code, along with macro expansion information."] # [derive (Copy , Clone)] pub struct Span { inner : imp :: Span , _marker : ProcMacroAutoTraits , }