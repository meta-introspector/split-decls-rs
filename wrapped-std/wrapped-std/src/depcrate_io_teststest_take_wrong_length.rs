// Generated macro for test_take_wrong_length (function)
macro_rules! Depcrate_io_teststest_take_wrong_length {
() => {
// Module: crate::io::tests
// Provides: {"test_take_wrong_length"}
// Dependencies: {}
# [test] # [should_panic = "number of read bytes exceeds limit"] fn test_take_wrong_length () { struct LieAboutSize (bool) ; impl Read for LieAboutSize { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if core :: mem :: take (& mut self . 0) { Ok (buf . len () + 1) } else { Ok (buf . len ()) } } } let mut buffer = vec ! [0 ; 4] ; let mut reader = LieAboutSize (true) . take (4) ; let _ = reader . read (& mut buffer [..]) ; }
};
}
