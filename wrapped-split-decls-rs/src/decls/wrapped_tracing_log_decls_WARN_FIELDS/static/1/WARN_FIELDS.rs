use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static WARN_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& WARN_CS)) ;
}