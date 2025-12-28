use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum DecouplingStrategy { TraitObject , GenericBound , DependencyInject , }
}