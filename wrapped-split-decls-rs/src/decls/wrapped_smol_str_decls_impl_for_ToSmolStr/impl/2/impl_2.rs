use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ToSmolStr for T where T : fmt :: Display + ? Sized , { fn to_smolstr (& self) -> SmolStr { format_smolstr ! ("{}" , self) } }
}