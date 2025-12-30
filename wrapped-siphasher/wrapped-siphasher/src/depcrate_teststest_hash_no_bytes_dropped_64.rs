// Generated macro for test_hash_no_bytes_dropped_64 (function)
macro_rules! Depcrate_teststest_hash_no_bytes_dropped_64 {
() => {
// Module: crate::tests
// Provides: {"test_hash_no_bytes_dropped_64"}
// Dependencies: {}
# [test] fn test_hash_no_bytes_dropped_64 () { let val = 0xdead_beef_dead_beef_u64 ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 0))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 1))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 2))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 3))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 4))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 5))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 6))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 7))) ; fn zero_byte (val : u64 , byte : usize) -> u64 { assert ! (byte < 8) ; val & ! (0xff << (byte * 8)) } }
};
}
