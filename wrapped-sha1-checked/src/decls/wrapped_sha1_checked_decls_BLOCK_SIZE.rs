use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const BLOCK_SIZE: usize = <sha1::block_api::Sha1Core as BlockSizeUser>::BlockSize::USIZE;
