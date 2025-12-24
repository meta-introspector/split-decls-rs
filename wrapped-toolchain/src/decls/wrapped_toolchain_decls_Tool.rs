use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone)]
pub enum Tool {
    Cargo,
    Rustc,
    Rustup,
    Rustfmt,
}
