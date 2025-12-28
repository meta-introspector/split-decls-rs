use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HeckeOperator for DummyHeckeOperator { fn apply_transformation (& self , mut declaration : Declaration) -> Declaration { declaration . name = format ! ("transformed_{}" , declaration . name) ; if let Some (mut factors) = declaration . monster_factors . take () { factors . push (7) ; declaration . monster_factors = Some (factors) ; } else { declaration . monster_factors = Some (vec ! [7]) ; } declaration } }
}