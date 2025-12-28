use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx > fmt :: Display for LazyDefPathStr < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . tcx . def_path_str (self . def_id)) } }