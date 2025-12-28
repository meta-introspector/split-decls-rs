use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) fn do_in_place_scope_fifo < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& ScopeFifo < 'scope >) -> R , { let thread = unsafe { WorkerThread :: current () . as_ref () } ; let scope = ScopeFifo :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }