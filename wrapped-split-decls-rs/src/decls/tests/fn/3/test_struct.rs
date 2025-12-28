use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_struct () { check_round_trip (vec ! [Struct { a : () , b : 10 , c : 11 , d : 12 , e : 13 , f : 14 , g : 15 , h : 16 , i : 17 , j : 18 , k : 19 , l : 'x' , m : "abc" . to_string () , p : false , q : None , }]) ; check_round_trip (vec ! [Struct { a : () , b : 101 , c : 111 , d : 121 , e : 131 , f : 141 , g : - 15 , h : - 16 , i : - 17 , j : - 18 , k : - 19 , l : 'y' , m : "def" . to_string () , p : true , q : Some (1234567) , }]) ; }