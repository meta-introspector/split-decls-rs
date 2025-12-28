use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl private { fn ident (& self) -> Ident { Ident :: new (concat ! ("__private" , env ! ("CARGO_PKG_VERSION_PATCH")) , Span :: call_site () ,) } }
}