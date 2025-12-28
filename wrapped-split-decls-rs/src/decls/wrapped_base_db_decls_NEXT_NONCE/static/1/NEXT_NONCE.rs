use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static NEXT_NONCE : AtomicUsize = AtomicUsize :: new (0) ;