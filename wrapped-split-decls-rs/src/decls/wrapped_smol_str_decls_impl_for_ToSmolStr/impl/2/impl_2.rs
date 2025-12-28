use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ToSmolStr for T where T : fmt :: Display + ? Sized , { fn to_smolstr (& self) -> SmolStr { format_smolstr ! ("{}" , self) } }