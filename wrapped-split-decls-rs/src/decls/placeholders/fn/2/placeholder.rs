use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn placeholder (kind : AstFragmentKind , id : ast :: NodeId , vis : Option < ast :: Visibility > ,) -> AstFragment { panic ! ("placeholder function called for kind: {:?}" , kind) ; }
}