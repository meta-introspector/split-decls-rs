// Generated macro for test (module)
macro_rules! Depcrate_arrayvectest {
() => {
// Module: crate::arrayvec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn retain_mut_empty_vec () { let mut av : ArrayVec < [i32 ; 4] > = ArrayVec :: new () ; av . retain_mut (| & mut x | x % 2 == 0) ; assert_eq ! (av . len () , 0) ; } # [test] fn retain_mut_all_elements () { let mut av : ArrayVec < [i32 ; 4] > = array_vec ! ([i32 ; 4] => 2 , 4 , 6 , 8) ; av . retain_mut (| & mut x | x % 2 == 0) ; assert_eq ! (av . len () , 4) ; assert_eq ! (av . as_slice () , & [2 , 4 , 6 , 8]) ; } # [test] fn retain_mut_some_elements () { let mut av : ArrayVec < [i32 ; 4] > = array_vec ! ([i32 ; 4] => 1 , 2 , 3 , 4) ; av . retain_mut (| & mut x | x % 2 == 0) ; assert_eq ! (av . len () , 2) ; assert_eq ! (av . as_slice () , & [2 , 4]) ; } # [test] fn retain_mut_no_elements () { let mut av : ArrayVec < [i32 ; 4] > = array_vec ! ([i32 ; 4] => 1 , 3 , 5 , 7) ; av . retain_mut (| & mut x | x % 2 == 0) ; assert_eq ! (av . len () , 0) ; } # [test] fn retain_mut_zero_capacity () { let mut av : ArrayVec < [i32 ; 0] > = ArrayVec :: new () ; av . retain_mut (| & mut x | x % 2 == 0) ; assert_eq ! (av . len () , 0) ; } }
};
}
