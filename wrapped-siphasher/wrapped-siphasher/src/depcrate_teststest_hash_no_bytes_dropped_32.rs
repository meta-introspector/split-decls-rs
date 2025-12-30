// Generated macro for test_hash_no_bytes_dropped_32 (function)
macro_rules! Depcrate_teststest_hash_no_bytes_dropped_32 {
() => {
// Module: crate::tests
// Provides: {"test_hash_no_bytes_dropped_32"}
// Dependencies: {}
# [test] fn test_hash_no_bytes_dropped_32 () { let val = 0xdeadbeef_u32 ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 0))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 1))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 2))) ; assert_ne ! (hash (& val) , hash (& zero_byte (val , 3))) ; fn zero_byte (val : u32 , byte : usize) -> u32 { assert ! (byte < 4) ; val & ! (0xff << (byte * 8)) } }
};
}
