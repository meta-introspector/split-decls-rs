use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static SERVICE_CONTEXT : RwLock < ServiceContext > = RwLock :: new (ServiceContext (std :: ptr :: null ())) ;
}