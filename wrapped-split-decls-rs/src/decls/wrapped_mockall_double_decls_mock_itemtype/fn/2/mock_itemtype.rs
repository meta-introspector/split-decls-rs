use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn mock_itemtype (orig : & mut ItemType) { match & mut * orig . ty { Type :: Path (tp) => { let ident = & tp . path . segments . last_mut () . unwrap () . ident ; tp . path . segments . last_mut () . unwrap () . ident = mock_ident (ident) ; } x => compile_error (x . span () , "Only path types may be doubled") , } }
}