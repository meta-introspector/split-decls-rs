use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > Borrow < IndexSlice < I , T > > for IndexVec < I , T > { fn borrow (& self) -> & IndexSlice < I , T > { self } }