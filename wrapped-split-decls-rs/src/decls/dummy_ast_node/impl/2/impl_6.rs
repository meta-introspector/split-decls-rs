use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DummyAstNode for ast :: Expr { # [doc = " Returns a dummy `ast::Expr` (expression) node."] fn dummy () -> Self { ast :: Expr { id : DUMMY_NODE_ID , kind : ast :: ExprKind :: Tup (Default :: default ()) , span : Span :: default () , attrs : Default :: default () , tokens : Default :: default () , } } }