use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl BlockSizeUser for Sha1 {
    type BlockSize = U64;
}
