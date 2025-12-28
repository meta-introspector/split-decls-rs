use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < F > ThreadSpawn for CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { private_impl ! { } # [inline] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { (self . 0) (thread) } }
}