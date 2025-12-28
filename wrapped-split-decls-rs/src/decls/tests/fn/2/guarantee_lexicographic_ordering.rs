use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn guarantee_lexicographic_ordering () { let abis = ExternAbi :: ALL_VARIANTS ; let mut sorted_abis = abis . to_vec () ; sorted_abis . sort_unstable () ; assert_eq ! (abis , sorted_abis) ; }
}