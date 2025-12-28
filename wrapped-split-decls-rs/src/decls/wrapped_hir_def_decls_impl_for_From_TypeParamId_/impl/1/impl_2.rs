use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < TypeParamId > for TypeOrConstParamId { fn from (it : TypeParamId) -> Self { it . 0 } }