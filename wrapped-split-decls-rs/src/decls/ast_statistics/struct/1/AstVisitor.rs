use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct AstVisitor < 'a > { ast_stats : & 'a mut AstStatistics , rdf_state : & 'a mut RdfStateMachine , }