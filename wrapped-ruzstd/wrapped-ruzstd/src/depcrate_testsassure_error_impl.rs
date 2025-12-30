// Generated macro for assure_error_impl (function)
macro_rules! Depcrate_testsassure_error_impl {
() => {
// Module: crate::tests
// Provides: {"assure_error_impl"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] # [allow (dead_code)] fn assure_error_impl () { use crate :: decoding :: errors :: FrameDecoderError ; let _err : & dyn std :: error :: Error = & FrameDecoderError :: NotYetInitialized ; }
};
}
