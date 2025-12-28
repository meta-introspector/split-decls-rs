use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Convert value to [`SmolStr`] using [`fmt::Display`], potentially without allocating."] # [doc = ""] # [doc = " Almost identical to [`ToString`], but converts to `SmolStr` instead."] pub trait ToSmolStr { fn to_smolstr (& self) -> SmolStr ; }
}