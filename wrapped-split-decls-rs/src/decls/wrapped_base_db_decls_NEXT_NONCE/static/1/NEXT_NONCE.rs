use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static NEXT_NONCE : AtomicUsize = AtomicUsize :: new (0) ;
}