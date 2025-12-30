// Generated macro for test_vec (function)
macro_rules! Depcratetest_vec {
() => {
// Module: crate
// Provides: {"test_vec"}
// Dependencies: {}
pub fn test_vec < A : Allocator > (alloc : A) { let mut vec = Vec :: < u8 , A > :: new_in (alloc) ; vec . push (1) ; vec . push (2) ; vec . shrink_to_fit () ; vec . push (3) ; vec . resize (10 , 0xba) ; vec . shrink_to_fit () ; vec . resize (12467 , 0xfe) ; drop (vec) ; }
};
}
