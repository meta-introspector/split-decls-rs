use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for SuggestionKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SuggestionKind :: Normal => write ! (f , "normal") , SuggestionKind :: Short => write ! (f , "short") , SuggestionKind :: Hidden => write ! (f , "hidden") , SuggestionKind :: Verbose => write ! (f , "verbose") , SuggestionKind :: ToolOnly => write ! (f , "tool-only") , } } }
}