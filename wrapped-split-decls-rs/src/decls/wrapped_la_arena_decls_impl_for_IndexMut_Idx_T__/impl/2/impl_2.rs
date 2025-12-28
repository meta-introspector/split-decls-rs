use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > IndexMut < Idx < T > > for Arena < T > { fn index_mut (& mut self , idx : Idx < T >) -> & mut T { let idx = idx . into_raw () . 0 as usize ; & mut self . data [idx] } }