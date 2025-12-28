use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_size () { assert_eq ! (core :: mem :: size_of ::< TinyStr4 > () , core :: mem :: size_of ::< Option < TinyStr4 >> ()) ; assert_eq ! (core :: mem :: size_of ::< TinyStr8 > () , core :: mem :: size_of ::< Option < TinyStr8 >> ()) ; }
}