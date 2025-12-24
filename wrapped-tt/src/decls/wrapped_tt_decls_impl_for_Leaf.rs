use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> Leaf<S> {
    pub fn span(&self) -> &S {
        match self {
            Leaf::Literal(it) => &it.span,
            Leaf::Punct(it) => &it.span,
            Leaf::Ident(it) => &it.span,
        }
    }
}
