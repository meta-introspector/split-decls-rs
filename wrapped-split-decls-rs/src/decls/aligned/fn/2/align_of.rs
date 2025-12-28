use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns the ABI-required minimum alignment of a type in bytes."] # [doc = ""] # [doc = " This is equivalent to [`align_of`], but also works for some unsized"] # [doc = " types (e.g. slices or rustc's `List`s)."] pub const fn align_of < T : ? Sized + Aligned > () -> Alignment { T :: ALIGN }
}