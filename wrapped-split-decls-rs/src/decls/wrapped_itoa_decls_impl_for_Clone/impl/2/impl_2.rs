use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [allow (clippy :: non_canonical_clone_impl)] impl Clone for Buffer { # [inline] fn clone (& self) -> Self { Buffer :: new () } }
}