use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl RenderedExpandError { const GENERAL_KIND : & str = "macro-error" ; const DISABLED : & str = "proc-macro-disabled" ; const ATTR_EXP_DISABLED : & str = "attribute-expansion-disabled" ; }