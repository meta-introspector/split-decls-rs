use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AttrIdGenerator (AtomicU32) ;