// Generated macro for tests (module)
macro_rules! Depcrate_msgs_codectests {
() => {
// Module: crate::msgs::codec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: prelude :: v1 :: * ; use std :: vec ; use super :: * ; # [test] fn interrupted_length_prefixed_buffer_leaves_maximum_length () { let mut buf = Vec :: new () ; let nested = LengthPrefixedBuffer :: new (ListLength :: U16 , & mut buf) ; nested . buf . push (0xaa) ; assert_eq ! (nested . buf , & vec ! [0xff , 0xff , 0xaa]) ; drop (nested) ; assert_eq ! (buf , vec ! [0x00 , 0x01 , 0xaa]) ; } }
};
}
