use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " ////////////////////////////////////////////////////////////////////////"] unsafe fn main_loop (thread : ThreadBuilder) { let worker_thread = & WorkerThread :: from (thread) ; unsafe { WorkerThread :: set_current (worker_thread) } ; let registry = & * worker_thread . registry ; let index = worker_thread . index ; unsafe { Latch :: set (& registry . thread_infos [index] . primed) } ; let abort_guard = unwind :: AbortIfPanic ; if let Some (ref handler) = registry . start_handler { registry . catch_unwind (| | handler (index)) ; } unsafe { worker_thread . wait_until_out_of_work () } ; mem :: forget (abort_guard) ; if let Some (ref handler) = registry . exit_handler { registry . catch_unwind (| | handler (index)) ; } registry . release_thread () ; }
}