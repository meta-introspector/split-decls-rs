use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static RUSTC_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | build_lints_map (DEFAULT_LINTS , DEFAULT_LINT_GROUPS , "")) ;
}