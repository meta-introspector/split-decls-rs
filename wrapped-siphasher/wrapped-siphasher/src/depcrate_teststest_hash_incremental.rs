// Generated macro for test_hash_incremental (function)
macro_rules! Depcrate_teststest_hash_incremental {
() => {
// Module: crate::tests
// Provides: {"test_hash_incremental"}
// Dependencies: {}
# [test] fn test_hash_incremental () { let array1 : & [u8] = & [1 , 2 , 3] ; let array2 : & [u8] = & [4 , 5 , 6] ; let key : & [u8 ; 16] = & [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16] ; let mut hasher = SipHasher13 :: new_with_key (key) ; hasher . write (array1) ; hasher . write (array2) ; let h = hasher . finish () ; _ = h ; }
};
}
