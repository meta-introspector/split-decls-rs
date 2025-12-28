use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Types which use key for initialization."] # [doc = ""] # [doc = " Generally it's used indirectly via [`KeyInit`] or [`KeyIvInit`]."] pub trait KeySizeUser { # [doc = " Key size in bytes."] type KeySize : ArraySize ; # [doc = " Return key size in bytes."] # [inline (always)] fn key_size () -> usize { Self :: KeySize :: USIZE } }
}