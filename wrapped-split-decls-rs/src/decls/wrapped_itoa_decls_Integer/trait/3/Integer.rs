use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " An integer that can be written into an [`itoa::Buffer`][Buffer]."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of itoa."] pub trait Integer : private :: Sealed { # [doc = " The maximum length of string that formatting an integer of this type can"] # [doc = " produce on the current target platform."] const MAX_STR_LEN : usize ; }
}