use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn binary_search_slice_test () { let map = test_map () ; assert_eq ! (binary_search_slice (& map , get_key , & 0) , & [(0 , "zero")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 1) , & []) ; assert_eq ! (binary_search_slice (& map , get_key , & 3) , & [(3 , "three-a") , (3 , "three-b")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 22) , & [(22 , "twenty-two")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 23) , & []) ; }