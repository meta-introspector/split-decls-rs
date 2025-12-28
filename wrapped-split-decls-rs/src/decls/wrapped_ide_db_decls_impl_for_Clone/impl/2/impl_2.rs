use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Clone for RootDatabase { fn clone (& self) -> Self { Self { storage : self . storage . clone () , files : self . files . clone () , crates_map : self . crates_map . clone () , nonce : Nonce :: new () , } } }
}