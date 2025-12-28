use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: fmt :: Display for VersionInfo { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let hash = self . commit_hash . clone () . unwrap_or_default () ; let hash_trimmed = hash . trim () ; let date = self . commit_date . clone () . unwrap_or_default () ; let date_trimmed = date . trim () ; if (hash_trimmed . len () + date_trimmed . len ()) > 0 { write ! (f , "{} {}.{}.{} ({hash_trimmed} {date_trimmed})" , self . crate_name , self . major , self . minor , self . patch ,) ? ; } else { write ! (f , "{} {}.{}.{}" , self . crate_name , self . major , self . minor , self . patch) ? ; } Ok (()) } }
}