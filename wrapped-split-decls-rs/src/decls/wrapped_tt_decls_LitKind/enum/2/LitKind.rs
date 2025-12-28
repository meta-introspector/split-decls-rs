use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum LitKind { Byte , Char , Integer , Float , Str , StrRaw (u8) , ByteStr , ByteStrRaw (u8) , CStr , CStrRaw (u8) , Err (()) , }