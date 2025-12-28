use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " By impl Deref, we can access the wrapped Ident as if it were a normal Ident"] # [doc = " such as `norm_ident.name` instead of `norm_ident.0.name`."] impl Deref for Macros20NormalizedIdent { type Target = Ident ; fn deref (& self) -> & Self :: Target { & self . 0 } }