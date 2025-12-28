use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ParseResultExt < T > for OkParse < T > { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (OkParse (op (self . 0))) } }