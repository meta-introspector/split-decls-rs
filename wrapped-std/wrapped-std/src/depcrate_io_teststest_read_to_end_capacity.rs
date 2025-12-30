// Generated macro for test_read_to_end_capacity (function)
macro_rules! Depcrate_io_teststest_read_to_end_capacity {
() => {
// Module: crate::io::tests
// Provides: {"test_read_to_end_capacity"}
// Dependencies: {}
# [test] fn test_read_to_end_capacity () -> io :: Result < () > { let input = & b"foo" [..] ; let mut vec1 = Vec :: with_capacity (input . len ()) ; ExampleSliceReader { slice : input } . read_to_end (& mut vec1) ? ; assert_eq ! (vec1 . len () , input . len ()) ; assert_eq ! (vec1 . capacity () , input . len () , "did not allocate more") ; Ok (()) }
};
}
