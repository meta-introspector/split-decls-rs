use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Variance { Bivariant , Covariant , Contravariant , Invariant , }
}