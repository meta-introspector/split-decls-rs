use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > BorrowMut < [T] > for ThinVec < T > { fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }