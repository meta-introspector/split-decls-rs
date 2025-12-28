use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Serialize , Deserialize)] struct RdfState { last_query : String , query_history : Vec < String > , bookmarks : HashMap < String , String > , }
}