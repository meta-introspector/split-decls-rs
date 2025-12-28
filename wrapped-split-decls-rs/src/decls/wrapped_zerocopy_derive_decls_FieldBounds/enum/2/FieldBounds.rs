use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum FieldBounds < 'a > { None , All (& 'a [TraitBound]) , Trailing (& 'a [TraitBound]) , Explicit (Vec < WherePredicate >) , }
}