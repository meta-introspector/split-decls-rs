use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ExpnKind { pub fn descr (& self) -> String { match * self { ExpnKind :: Root => kw :: PathRoot . to_string () , ExpnKind :: Macro (macro_kind , name) => match macro_kind { MacroKind :: Bang => format ! ("{name}!") , MacroKind :: Attr => format ! ("#[{name}]") , MacroKind :: Derive => format ! ("#[derive({name})]") , } , ExpnKind :: AstPass (kind) => kind . descr () . to_string () , ExpnKind :: Desugaring (kind) => format ! ("desugaring of {}" , kind . descr ()) , } } }