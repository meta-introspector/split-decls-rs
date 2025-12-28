use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_string () { let vec = vec ! ["abcbuÖeiovÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmeabpnvapeapmaebn" . to_string () , "abcbuÖganeiÄnameÜavmpßvmea€µsbpnvapeapmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µsbpmaebn" . to_string () , "abcbuÖganeiovÄnameÜavmpßvmea€µnvapeapmaebn" . to_string () ,] ; check_round_trip (vec) ; }
}