use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: max_num_threads");
# [doc = " Returns the maximum number of threads that Rayon supports in a single thread pool."] # [doc = ""] # [doc = " If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting"] # [doc = " the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum."] # [doc = ""] # [doc = " The value may vary between different targets, and is subject to change in new Rayon versions."] pub fn max_num_threads () -> usize { crate :: sleep :: THREADS_MAX }
}