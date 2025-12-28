use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static ICE_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ;
}