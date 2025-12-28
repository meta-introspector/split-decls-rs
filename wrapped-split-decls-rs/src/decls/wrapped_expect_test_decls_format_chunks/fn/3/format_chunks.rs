use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn format_chunks (chunks : Vec < dissimilar :: Chunk >) -> String { let mut buf = String :: new () ; for chunk in chunks { let formatted = match chunk { dissimilar :: Chunk :: Equal (text) => text . into () , dissimilar :: Chunk :: Delete (text) => format ! ("\x1b[4m\x1b[31m{}\x1b[0m" , text) , dissimilar :: Chunk :: Insert (text) => format ! ("\x1b[4m\x1b[32m{}\x1b[0m" , text) , } ; buf . push_str (& formatted) ; } buf }
}