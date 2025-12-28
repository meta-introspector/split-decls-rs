use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a job stored in the heap. Used to implement"] # [doc = " `scope`. Unlike `StackJob`, when executed, `HeapJob` simply"] # [doc = " invokes a closure, which then triggers the appropriate logic to"] # [doc = " signal that the job executed."] # [doc = ""] # [doc = " (Probably `StackJob` should be refactored in a similar fashion.)"] pub (super) struct HeapJob < BODY > where BODY : FnOnce (JobRefId) + Send , { job : BODY , tlv : Tlv , }
}