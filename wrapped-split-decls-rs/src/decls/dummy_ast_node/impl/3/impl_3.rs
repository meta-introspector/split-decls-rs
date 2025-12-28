use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DummyAstNode for ast :: Crate { # [doc = " Returns a dummy `ast::Crate` node."] fn dummy () -> Self { ast :: Crate { attrs : Default :: default () , items : Default :: default () , spans : ast :: ModSpans { inner_span : Span :: default () , .. Default :: default () } , id : DUMMY_NODE_ID , is_placeholder : Default :: default () , } } }
}