use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn provide (providers : & mut Providers) { * providers = Providers { mir_borrowck , .. * providers } ; }
}