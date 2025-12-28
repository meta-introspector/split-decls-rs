use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: hash :: Hash for LintId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let ptr = self . lint as * const Lint ; ptr . hash (state) ; } }
}