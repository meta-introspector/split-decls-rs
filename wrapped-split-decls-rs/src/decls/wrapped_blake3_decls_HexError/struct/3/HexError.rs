use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The error type for [`Hash::from_hex`]."] # [doc = ""] # [doc = " The `.to_string()` representation of this error currently distinguishes between bad length"] # [doc = " errors and bad character errors. This is to help with logging and debugging, but it isn't a"] # [doc = " stable API detail, and it may change at any time."] # [derive (Clone , Debug)] pub struct HexError (HexErrorInner) ;
}