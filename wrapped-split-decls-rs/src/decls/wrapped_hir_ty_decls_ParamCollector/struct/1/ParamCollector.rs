use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ParamCollector { params : FxHashSet < TypeOrConstParamId > , }