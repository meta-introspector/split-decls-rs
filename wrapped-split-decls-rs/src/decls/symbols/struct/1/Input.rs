use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Input { keywords : Punctuated < Keyword , Token ! [,] > , symbols : Punctuated < Symbol , Token ! [,] > , }