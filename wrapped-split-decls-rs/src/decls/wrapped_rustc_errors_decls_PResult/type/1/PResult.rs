use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type PResult < 'a , T > = Result < T , Diag < 'a > > ;
}