use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S > SynToken < S > { fn token (& self) -> & SyntaxToken { match self { SynToken :: Ordinary (it) | SynToken :: Punct { token : it , offset : _ , } => it , SynToken :: Leaf (_) => unreachable ! () , } } }