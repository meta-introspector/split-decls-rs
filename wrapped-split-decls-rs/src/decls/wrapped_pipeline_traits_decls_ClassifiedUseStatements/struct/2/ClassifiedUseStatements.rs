use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct ClassifiedUseStatements (pub Vec < UseStatement > , pub HashMap < String , Vec < String > >) ;
}