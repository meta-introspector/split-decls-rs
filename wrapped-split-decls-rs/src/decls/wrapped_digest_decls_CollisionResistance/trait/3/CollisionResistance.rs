use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Types with a certain collision resistance."] pub trait CollisionResistance { # [doc = " Collision resistance in bytes."] # [doc = ""] # [doc = " This applies to an output size of at least `2 * CollisionResistance` bytes."] # [doc = " For a smaller output size collision resistance can be usually calculated as"] # [doc = " `min(CollisionResistance, OutputSize / 2)`."] type CollisionResistance : Unsigned ; }
}