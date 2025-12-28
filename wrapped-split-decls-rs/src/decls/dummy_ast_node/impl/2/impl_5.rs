use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DummyAstNode for ast :: Pat { # [doc = " Returns a dummy `ast::Pat` (pattern) node."] fn dummy () -> Self { ast :: Pat { id : DUMMY_NODE_ID , kind : PatKind :: Wild , span : Span :: default () , tokens : Default :: default () , } } }
}