use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct AstVisitor < 'a > { ast_stats : & 'a mut AstStatistics , rdf_state : & 'a mut RdfStateMachine , }
}