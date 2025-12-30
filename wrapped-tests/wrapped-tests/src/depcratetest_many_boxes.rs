// Generated macro for test_many_boxes (function)
macro_rules! Depcratetest_many_boxes {
() => {
// Module: crate
// Provides: {"test_many_boxes"}
// Dependencies: {}
pub fn test_many_boxes < A : Allocator + Copy > (alloc : A) { let mut boxes = Vec :: new_in (alloc) ; for i in 0 .. 15 { boxes . push (Box :: new_in (i , alloc)) ; } for i in 0 .. 15 { assert_eq ! (* boxes [i] , i) ; } drop (boxes) ; }
};
}
