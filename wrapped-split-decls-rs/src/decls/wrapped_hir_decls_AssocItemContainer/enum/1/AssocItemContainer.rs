use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum AssocItemContainer { Trait (Trait) , Impl (Impl) , }
}