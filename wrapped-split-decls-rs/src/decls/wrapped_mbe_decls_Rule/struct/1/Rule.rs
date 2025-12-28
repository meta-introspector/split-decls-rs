use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug , PartialEq , Eq)] struct Rule { lhs : MetaTemplate , rhs : MetaTemplate , }
}