use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : RefCnt , S : Default + Strategy < T > > From < T > for ArcSwapAny < T , S > { fn from (val : T) -> Self { Self :: with_strategy (val , S :: default ()) } }
}