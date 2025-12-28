use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A literal string (`\"hello\"`), byte string (`b\"hello\"`), character (`'a'`),"] # [doc = " byte character (`b'a'`), an integer or floating point number with or without"] # [doc = " a suffix (`1`, `1u8`, `2.3`, `2.3f32`)."] # [doc = ""] # [doc = " Boolean literals like `true` and `false` do not belong here, they are"] # [doc = " `Ident`s."] # [derive (Clone)] pub struct Literal { inner : imp :: Literal , _marker : ProcMacroAutoTraits , }