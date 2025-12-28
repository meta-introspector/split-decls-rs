use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (not (any (unix)) , ignore)] fn stack_overflow_crash () { let status = run_ignored ("run_with_small_stack") ; assert ! (! status . success ()) ; # [cfg (any (unix , windows))] assert_eq ! (status . code () , overflow_code ()) ; # [cfg (target_os = "linux")] assert ! (matches ! (status . signal () , Some (libc :: SIGABRT | libc :: SIGSEGV))) ; let status = run_ignored ("run_with_large_stack") ; assert_eq ! (status . code () , Some (0)) ; # [cfg (target_os = "linux")] assert_eq ! (status . signal () , None) ; }
}