use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait ByteWriter { fn write_u8 (& mut self , value : u8) -> Result < () > ; fn write_u16 (& mut self , value : u16) -> Result < () > ; fn write_u32 (& mut self , value : u32) -> Result < () > ; fn write_u64 (& mut self , value : u64) -> Result < () > ; }
}