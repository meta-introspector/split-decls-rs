use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static ICE_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ;