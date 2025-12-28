use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (debug_assertions)] impl Drop for FileEncoder { fn drop (& mut self) { if ! std :: thread :: panicking () { assert ! (self . finished) ; } } }
}