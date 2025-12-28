use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl JobFifo { pub (super) fn new () -> Self { JobFifo { inner : Injector :: new () } } pub (super) unsafe fn push (& self , job_ref : JobRef) -> JobRef { self . inner . push (job_ref) ; unsafe { JobRef :: new (self) } } }
}