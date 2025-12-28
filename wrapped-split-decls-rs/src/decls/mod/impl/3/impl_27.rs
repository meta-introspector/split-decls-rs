use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'scope > fmt :: Debug for ScopeFifo < 'scope > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ScopeFifo") . field ("num_fifos" , & self . fifos . len ()) . field ("pool_id" , & self . base . registry . id ()) . field ("panic" , & self . base . panic) . field ("job_completed_latch" , & self . base . job_completed_latch) . finish () } }
}