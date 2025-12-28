use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Index < Idx < T > > for Arena < T > { type Output = T ; fn index (& self , idx : Idx < T >) -> & T { let idx = idx . into_raw () . 0 as usize ; & self . data [idx] } }