use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < ThreadBuilder > for WorkerThread { fn from (thread : ThreadBuilder) -> Self { Self { worker : thread . worker , stealer : thread . stealer , fifo : JobFifo :: new () , index : thread . index , rng : XorShift64Star :: new () , registry : thread . registry , } } }