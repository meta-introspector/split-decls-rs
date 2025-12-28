use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AngleBracketedArg { pub fn span (& self) -> Span { match self { AngleBracketedArg :: Arg (arg) => arg . span () , AngleBracketedArg :: Constraint (constraint) => constraint . span , } } }
}