use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: spawn_broadcast_in");
# [doc = " Execute `op` on every thread in the pool. It will be executed on each"] # [doc = " thread when they have nothing else to do locally, before they try to"] # [doc = " steal work from other threads. This function returns immediately after"] # [doc = " injecting the jobs."] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn spawn_broadcast_in < OP > (op : OP , registry : & Arc < Registry >) where OP : Fn (BroadcastContext < '_ >) + Send + Sync + 'static , { let job = ArcJob :: new ({ let registry = Arc :: clone (registry) ; move | _ | { registry . catch_unwind (| | BroadcastContext :: with (& op)) ; registry . terminate () ; } }) ; let n_threads = registry . num_threads () ; let job_refs = (0 .. n_threads) . map (| _ | { registry . increment_terminate_count () ; ArcJob :: as_static_job_ref (& job) }) ; registry . inject_broadcast (job_refs) ; }
}