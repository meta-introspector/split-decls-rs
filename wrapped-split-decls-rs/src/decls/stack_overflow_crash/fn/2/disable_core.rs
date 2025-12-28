use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (unix)] fn disable_core () { unsafe { libc :: setrlimit (libc :: RLIMIT_CORE , & libc :: rlimit { rlim_cur : 0 , rlim_max : 0 }) ; } }