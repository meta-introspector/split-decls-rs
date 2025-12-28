use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: fmt :: Debug for MatchDebugInfo { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . matched { Ok (_) => writeln ! (f , "Node matched") ? , Err (reason) => writeln ! (f , "Node failed to match because: {}" , reason . reason) ? , } writeln ! (f , "============ AST ===========\n\
            {:#?}" , self . node) ? ; writeln ! (f , "========= PATTERN ==========") ? ; writeln ! (f , "{:#?}" , self . pattern) ? ; writeln ! (f , "============================") ? ; Ok (()) } }
}