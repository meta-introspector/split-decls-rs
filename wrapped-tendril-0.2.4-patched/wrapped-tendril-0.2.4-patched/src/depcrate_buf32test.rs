// Generated macro for test (module)
macro_rules! Depcrate_buf32test {
() => {
// Module: crate::buf32
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: Buf32 ; use std :: ptr ; # [test] fn smoke_test () { unsafe { let mut b = Buf32 :: with_capacity (0 , 0u8) ; assert_eq ! (b"" , b . data ()) ; b . grow (5) ; ptr :: copy_nonoverlapping (b"Hello" . as_ptr () , b . data_ptr () , 5) ; assert_eq ! (b"" , b . data ()) ; b . len = 5 ; assert_eq ! (b"Hello" , b . data ()) ; b . grow (1337) ; assert ! (b . cap >= 1337) ; assert_eq ! (b"Hello" , b . data ()) ; b . destroy () ; } } }
};
}
