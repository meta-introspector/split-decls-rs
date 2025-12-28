use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : sealed :: Integer > Extend for T { fn extend < U > (self) -> U where T : ExtendTarget < U > , { sealed :: ExtendTargetSealed :: extend (self) } }
}