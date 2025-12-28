use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl error :: Error for FromOsStrError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { None } }
}