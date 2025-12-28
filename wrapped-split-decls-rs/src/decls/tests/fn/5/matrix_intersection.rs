use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: matrix_intersection");
# [test] fn matrix_intersection () { let mut matrix : BitMatrix < usize , usize > = BitMatrix :: new (200 , 200) ; matrix . insert (2 , 3) ; matrix . insert (2 , 6) ; matrix . insert (2 , 10) ; matrix . insert (2 , 64) ; matrix . insert (2 , 65) ; matrix . insert (2 , 130) ; matrix . insert (2 , 160) ; matrix . insert (64 , 133) ; matrix . insert (65 , 2) ; matrix . insert (65 , 8) ; matrix . insert (65 , 10) ; matrix . insert (65 , 64) ; matrix . insert (65 , 68) ; matrix . insert (65 , 133) ; matrix . insert (65 , 160) ; let intersection = matrix . intersect_rows (2 , 64) ; assert ! (intersection . is_empty ()) ; let intersection = matrix . intersect_rows (2 , 65) ; assert_eq ! (intersection , & [10 , 64 , 160]) ; }
}