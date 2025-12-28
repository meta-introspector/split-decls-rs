use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PolyTraitRef { pub fn new (generic_params : ThinVec < GenericParam > , path : Path , modifiers : TraitBoundModifiers , span : Span , parens : Parens ,) -> Self { PolyTraitRef { bound_generic_params : generic_params , modifiers , trait_ref : TraitRef { path , ref_id : DUMMY_NODE_ID } , span , parens , } } }
}