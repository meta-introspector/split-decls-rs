use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn path_to_str_literal < P : AsRef < Path > > (path : P) -> Token { Token :: Literal (Lit :: Str (path . as_ref () . to_str () . unwrap () . to_owned () , StrStyle :: Cooked ,)) }