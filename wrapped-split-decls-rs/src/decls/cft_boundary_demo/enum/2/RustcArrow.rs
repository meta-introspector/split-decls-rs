use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum RustcArrow { FunctionCall , TypeReference , ImplArrow , TraitBound , LifetimeFlow , OwnershipMove , BorrowReference , ControlFlow , }
}