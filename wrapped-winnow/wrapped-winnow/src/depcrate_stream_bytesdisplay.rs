// Generated macro for display (module)
macro_rules! Depcrate_stream_bytesdisplay {
() => {
// Module: crate::stream::bytes
// Provides: {"display"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod display { use crate :: stream :: Bytes ; # [test] fn clean () { assert_eq ! (& format ! ("{}" , Bytes :: new (b"abc")) , "616263") ; assert_eq ! (& format ! ("{}" , Bytes :: new (b"\xf0\x28\x8c\xbc")) , "F0288CBC") ; } }
};
}
