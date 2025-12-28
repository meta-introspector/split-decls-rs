use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static THE_REGISTRY_SET : Once = Once :: new () ;
}