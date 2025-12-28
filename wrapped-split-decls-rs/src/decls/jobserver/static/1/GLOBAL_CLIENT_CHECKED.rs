use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static GLOBAL_CLIENT_CHECKED : OnceLock < Client > = OnceLock :: new () ;