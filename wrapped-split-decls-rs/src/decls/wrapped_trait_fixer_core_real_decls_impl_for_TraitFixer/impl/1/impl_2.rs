use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx , C > TraitFixer < 'tcx , C > where C : ConfigTrait { }
}