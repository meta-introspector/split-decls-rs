// Generated macro for test_hash_simple (function)
macro_rules! Depcrate_teststest_hash_simple {
() => {
// Module: crate::tests
// Provides: {"test_hash_simple"}
// Dependencies: {}
# [test] fn test_hash_simple () { let array : & [u8] = & [1 , 2 , 3] ; let key : & [u8 ; 16] = & [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16] ; let hasher = SipHasher13 :: new_with_key (key) ; let h = hasher . hash (array) ; _ = h ; }
};
}
