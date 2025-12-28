use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , S : Strategy < T > > Display for ArcSwapAny < T , S > where T : Display + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . load () . fmt (formatter) } }
}