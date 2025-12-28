use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DummyAstNode for ast :: Ty { # [doc = " Returns a dummy `ast::Ty` node."] fn dummy () -> Self { ast :: Ty { id : DUMMY_NODE_ID , kind : TyKind :: Dummy , span : Span :: default () , tokens : Default :: default () , } } }