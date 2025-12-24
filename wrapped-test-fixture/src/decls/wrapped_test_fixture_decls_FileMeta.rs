use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
struct FileMeta {
    path: String,
    krate: Option<(String, CrateOrigin, Option<String>)>,
    deps: Vec<String>,
    extern_prelude: Option<Vec<String>>,
    cfg: CfgOptions,
    edition: Edition,
    env: Env,
    introduce_new_source_root: Option<SourceRootKind>,
}
