use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L > Deref for LatchRef < '_ , L > { type Target = L ; fn deref (& self) -> & L { unsafe { & * self . inner } } }