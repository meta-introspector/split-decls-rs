use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Types which process data in blocks."] pub trait BlockSizeUser { # [doc = " Size of the block in bytes."] type BlockSize : BlockSizes ; # [doc = " Return block size in bytes."] # [inline (always)] fn block_size () -> usize { Self :: BlockSize :: USIZE } }
}