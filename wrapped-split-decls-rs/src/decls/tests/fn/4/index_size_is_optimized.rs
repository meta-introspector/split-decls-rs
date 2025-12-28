use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn index_size_is_optimized () { assert_eq ! (size_of ::< MyIdx > () , 4) ; assert_eq ! (size_of ::< Option < MyIdx >> () , 4) ; assert_eq ! (size_of ::< Option < Option < MyIdx >>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < MyIdx >>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < MyIdx >>>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < Option < MyIdx >>>>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < Option < Option < MyIdx >>>>>>> () , 8) ; }
}