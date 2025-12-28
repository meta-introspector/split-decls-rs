use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : sealed :: Integer > Truncate for T { fn truncate < U > (self) -> U where T : TruncateTarget < U > , { sealed :: TruncateTargetSealed :: truncate (self) } }