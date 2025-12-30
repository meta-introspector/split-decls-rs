// Generated macro for test_siphash128_simple (function)
macro_rules! Depcrate_tests128test_siphash128_simple {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_simple"}
// Dependencies: {}
# [test] fn test_siphash128_simple () { let array : & [u8] = & [1 , 2 , 3] ; let key : & [u8 ; 16] = & [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16] ; let hasher = SipHasher13 :: new_with_key (key) ; let h = hasher . hash (array) . as_bytes () ; _ = h ; }
};
}
