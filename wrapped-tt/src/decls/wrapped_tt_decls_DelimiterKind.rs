use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DelimiterKind {
    Parenthesis,
    Brace,
    Bracket,
    Invisible,
}
