use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Punct<S> {
    pub char: char,
    pub spacing: Spacing,
    pub span: S,
}
