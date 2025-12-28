use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static ENV_LOCK : std :: sync :: Mutex < () > = std :: sync :: Mutex :: new (()) ;
}