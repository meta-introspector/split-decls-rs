use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > ParseResultExt < T > for ErrParse < T > { fn map < U , F > (self , _op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (ErrParse :: new (self . failures)) } }