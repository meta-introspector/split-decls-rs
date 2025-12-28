use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Thread-local variable wrapper"] # [doc = ""] # [doc = " See the [module-level documentation](index.html) for more."] pub struct ThreadLocal < T : Send > { # [doc = " The buckets in the thread local. The nth bucket contains `2^n`"] # [doc = " elements. Each bucket is lazily allocated."] buckets : [AtomicPtr < Entry < T > > ; BUCKETS] , # [doc = " The number of values in the thread local. This can be less than the real number of values,"] # [doc = " but is never more."] values : AtomicUsize , }