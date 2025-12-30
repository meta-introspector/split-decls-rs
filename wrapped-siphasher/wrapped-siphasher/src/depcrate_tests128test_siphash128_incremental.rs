// Generated macro for test_siphash128_incremental (function)
macro_rules! Depcrate_tests128test_siphash128_incremental {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_incremental"}
// Dependencies: {}
# [test] fn test_siphash128_incremental () { let array1 : & [u8] = & [1 , 2 , 3] ; let array2 : & [u8] = & [4 , 5 , 6] ; let key : & [u8 ; 16] = & [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16] ; let mut hasher = SipHasher13 :: new_with_key (key) ; hasher . write (array1) ; hasher . write (array2) ; let h = hasher . finish128 () . as_bytes () ; _ = h ; }
};
}
