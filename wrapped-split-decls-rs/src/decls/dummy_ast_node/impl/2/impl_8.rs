use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DummyAstNode for AstNodeWrapper < ast :: Expr , MethodReceiverTag > { # [doc = " Returns a dummy `AstNodeWrapper` for a method receiver expression."] fn dummy () -> Self { AstNodeWrapper :: new (ast :: Expr :: dummy () , MethodReceiverTag) } }
}