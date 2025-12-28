use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Debug for Diff { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . hunks ()) . finish () } }