use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx , C > TraitFixer < 'tcx , C > where C : ConfigTrait { }