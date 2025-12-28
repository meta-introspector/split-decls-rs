use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] enum SynToken < S > { Ordinary (SyntaxToken) , Punct { token : SyntaxToken , offset : usize } , Leaf (tt :: Leaf < S >) , }