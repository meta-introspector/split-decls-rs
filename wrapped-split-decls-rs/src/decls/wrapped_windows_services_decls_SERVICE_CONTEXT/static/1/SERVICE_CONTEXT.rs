use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static SERVICE_CONTEXT : RwLock < ServiceContext > = RwLock :: new (ServiceContext (std :: ptr :: null ())) ;