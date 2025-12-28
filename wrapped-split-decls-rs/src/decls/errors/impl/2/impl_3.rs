use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Display for PagerankError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { PagerankError :: CapacityError (msg) => write ! (f , "{}" , msg) , } } }