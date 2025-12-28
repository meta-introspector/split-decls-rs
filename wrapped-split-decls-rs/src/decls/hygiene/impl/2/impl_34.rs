use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Transparency { pub fn fallback (macro_rules : bool) -> Self { if macro_rules { Transparency :: SemiOpaque } else { Transparency :: Opaque } } }