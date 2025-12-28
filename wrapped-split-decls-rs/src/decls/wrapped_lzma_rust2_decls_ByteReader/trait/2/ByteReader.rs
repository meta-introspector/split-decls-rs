use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait ByteReader { fn read_u8 (& mut self) -> Result < u8 > ; fn read_u16 (& mut self) -> Result < u16 > ; fn read_u16_be (& mut self) -> Result < u16 > ; fn read_u32 (& mut self) -> Result < u32 > ; fn read_u32_be (& mut self) -> Result < u32 > ; fn read_u64 (& mut self) -> Result < u64 > ; }
}