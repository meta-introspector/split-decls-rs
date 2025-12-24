use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The name of rustc's own place to organize libraries.
///
/// Used to be `rustc`, now the default is `rustlib`.
const RUST_LIB_DIR: &str = "rustlib";
