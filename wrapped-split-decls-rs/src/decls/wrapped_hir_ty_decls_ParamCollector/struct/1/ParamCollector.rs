use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ParamCollector { params : FxHashSet < TypeOrConstParamId > , }
}