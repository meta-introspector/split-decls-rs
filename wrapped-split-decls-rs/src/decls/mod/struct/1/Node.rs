use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Node < N > { first_edge : [EdgeIndex ; 2] , pub data : N , }