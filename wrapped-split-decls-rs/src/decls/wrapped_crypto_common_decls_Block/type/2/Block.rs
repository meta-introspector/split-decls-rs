use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Block on which [`BlockSizeUser`] implementors operate."] pub type Block < B > = Array < u8 , < B as BlockSizeUser > :: BlockSize > ;