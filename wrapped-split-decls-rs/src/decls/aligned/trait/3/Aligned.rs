use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " A type with a statically known alignment."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `Self::ALIGN` must be equal to the alignment of `Self`. For sized types it"] # [doc = " is [`align_of::<Self>()`], for unsized types it depends on the type, for"] # [doc = " example `[T]` has alignment of `T`."] # [doc = ""] # [doc = " [`align_of::<Self>()`]: align_of"] pub unsafe trait Aligned : PointeeSized { # [doc = " Alignment of `Self`."] const ALIGN : Alignment ; }
}