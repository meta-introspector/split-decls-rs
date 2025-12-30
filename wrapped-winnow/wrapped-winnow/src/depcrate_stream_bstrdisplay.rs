// Generated macro for display (module)
macro_rules! Depcrate_stream_bstrdisplay {
() => {
// Module: crate::stream::bstr
// Provides: {"display"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod display { use crate :: stream :: BStr ; # [test] fn clean () { assert_eq ! (& format ! ("{}" , BStr :: new (b"abc")) , "abc") ; assert_eq ! (& format ! ("{}" , BStr :: new (b"\xf0\x28\x8c\xbc")) , "�(��") ; } }
};
}
