use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_enum () { check_round_trip (vec ! [Enum :: Variant1 , Enum :: Variant2 (1 , 25) , Enum :: Variant3 { a : 3 , b : 'b' , c : false } , Enum :: Variant3 { a : - 4 , b : 'f' , c : true } ,]) ; }
}