use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ParenthesizedArgs { pub fn as_angle_bracketed_args (& self) -> AngleBracketedArgs { let args = self . inputs . iter () . cloned () . map (| input | AngleBracketedArg :: Arg (GenericArg :: Type (input))) . collect () ; AngleBracketedArgs { span : self . inputs_span , args } } }
}