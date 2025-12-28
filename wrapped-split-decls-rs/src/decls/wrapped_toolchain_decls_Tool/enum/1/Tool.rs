use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone)] pub enum Tool { Cargo , Rustc , Rustup , Rustfmt , }
}