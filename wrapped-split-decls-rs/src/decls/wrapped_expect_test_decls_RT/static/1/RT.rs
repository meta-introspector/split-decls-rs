use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static RT : Lazy < Mutex < Runtime > > = Lazy :: new (Default :: default) ;
}