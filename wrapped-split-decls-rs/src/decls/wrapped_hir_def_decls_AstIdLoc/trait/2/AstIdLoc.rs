use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait AstIdLoc { type Container ; type Ast : AstNode ; fn ast_id (& self) -> AstId < Self :: Ast > ; fn container (& self) -> Self :: Container ; }
}