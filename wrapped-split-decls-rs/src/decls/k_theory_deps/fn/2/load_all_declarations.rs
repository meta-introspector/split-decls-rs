use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn load_all_declarations () -> Result < Vec < Declaration > > { let mut decls = Vec :: new () ; if Path :: new ("output2") . exists () { load_declarations_recursive ("output2" , & mut decls) ? ; } Ok (decls) }
}