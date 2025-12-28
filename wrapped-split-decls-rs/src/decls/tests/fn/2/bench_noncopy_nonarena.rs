use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_noncopy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; }) }