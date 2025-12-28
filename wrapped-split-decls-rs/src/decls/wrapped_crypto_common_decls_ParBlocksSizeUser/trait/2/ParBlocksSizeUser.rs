use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Types which can process blocks in parallel."] pub trait ParBlocksSizeUser : BlockSizeUser { # [doc = " Number of blocks which can be processed in parallel."] type ParBlocksSize : ArraySize ; }
}