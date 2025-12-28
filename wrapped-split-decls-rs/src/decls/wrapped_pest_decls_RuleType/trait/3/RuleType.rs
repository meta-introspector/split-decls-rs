use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " A trait which parser rules must implement."] # [doc = ""] # [doc = " This trait is set up so that any struct that implements all of its required traits will"] # [doc = " automatically implement this trait as well."] # [doc = ""] # [doc = " This is essentially a [trait alias](https://github.com/rust-lang/rfcs/pull/1733). When trait"] # [doc = " aliases are implemented, this may be replaced by one."] pub trait RuleType : Copy + Debug + Eq + Hash + Ord { }
}