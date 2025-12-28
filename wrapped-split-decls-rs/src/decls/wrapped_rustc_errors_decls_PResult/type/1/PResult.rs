use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PResult < 'a , T > = Result < T , Diag < 'a > > ;