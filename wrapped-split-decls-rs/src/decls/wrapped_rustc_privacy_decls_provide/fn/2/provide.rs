use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn provide (providers : & mut Providers) { * providers = Providers { effective_visibilities , check_private_in_public , check_mod_privacy , .. * providers } ; }