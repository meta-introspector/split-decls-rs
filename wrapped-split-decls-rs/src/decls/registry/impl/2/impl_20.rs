use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < F > CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { pub (super) fn new (spawn : F) -> Self { CustomSpawn (spawn) } }
}