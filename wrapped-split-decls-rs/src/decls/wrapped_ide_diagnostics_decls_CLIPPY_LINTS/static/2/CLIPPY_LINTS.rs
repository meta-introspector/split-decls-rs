use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static CLIPPY_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | { build_lints_map (ide_db :: generated :: lints :: CLIPPY_LINTS , CLIPPY_LINT_GROUPS , "clippy::" ,) }) ;