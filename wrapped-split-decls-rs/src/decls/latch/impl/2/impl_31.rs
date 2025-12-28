use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L > LatchRef < '_ , L > { pub (super) fn new (inner : & L) -> LatchRef < '_ , L > { LatchRef { inner , marker : PhantomData } } }