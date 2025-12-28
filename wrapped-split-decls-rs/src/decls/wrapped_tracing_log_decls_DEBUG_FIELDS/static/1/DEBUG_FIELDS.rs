use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static DEBUG_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& DEBUG_CS)) ;
}