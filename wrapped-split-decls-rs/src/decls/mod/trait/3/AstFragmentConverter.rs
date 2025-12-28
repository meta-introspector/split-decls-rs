use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait AstFragmentConverter : Sized { type VisitOutputTy ; type FlatMapOutputTy : Default ; const KIND : AstFragmentKind ; fn to_annotatable (self) -> Annotatable ; fn fragment_to_visit_output (fragment : AstFragment) -> Self :: VisitOutputTy ; fn fragment_to_flat_map_output (fragment : AstFragment) -> Self :: FlatMapOutputTy ; }
}