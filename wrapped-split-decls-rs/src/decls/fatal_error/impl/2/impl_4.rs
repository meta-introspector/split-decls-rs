use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Display for FatalError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "fatal error") } }