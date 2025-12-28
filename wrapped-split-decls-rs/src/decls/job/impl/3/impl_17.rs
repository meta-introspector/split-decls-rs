use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L , F , R > Job for StackJob < L , F , R > where L : Latch + Sync , F : FnOnce (bool) -> R + Send , R : Send , { unsafe fn execute (this : * const ()) { let this = unsafe { & * (this as * const Self) } ; tlv :: set (this . tlv) ; let abort = unwind :: AbortIfPanic ; let func = unsafe { (* this . func . get ()) . take () . unwrap () } ; unsafe { (* this . result . get ()) = JobResult :: call (func) ; } unsafe { Latch :: set (& this . latch) ; } mem :: forget (abort) ; } }