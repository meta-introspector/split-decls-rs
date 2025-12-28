use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug)] enum HexErrorInner { InvalidByte (u8) , InvalidLen (usize) , }
}