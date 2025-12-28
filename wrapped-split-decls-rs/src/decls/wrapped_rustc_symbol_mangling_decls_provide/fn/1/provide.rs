use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn provide (providers : & mut Providers) { * providers = Providers { symbol_name : symbol_name_provider , .. * providers } ; }