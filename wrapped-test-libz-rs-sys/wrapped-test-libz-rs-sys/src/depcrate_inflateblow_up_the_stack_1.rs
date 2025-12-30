// Generated macro for blow_up_the_stack_1 (function)
macro_rules! Depcrate_inflateblow_up_the_stack_1 {
() => {
// Module: crate::inflate
// Provides: {"blow_up_the_stack_1"}
// Dependencies: {}
# [test] fn blow_up_the_stack_1 () { const INPUT : & [u8] = include_bytes ! ("test-data/blow_up_the_stack_1.gz") ; let mut output_ng = vec ! [0 ; INPUT . len () * 128] ; let mut output_rs = vec ! [0 ; INPUT . len () * 128] ; let config = InflateConfig :: default () ; let (_ , err) = crate :: helpers :: uncompress_slice_ng (& mut output_ng , INPUT , config) ; assert_eq ! (err , ReturnCode :: DataError) ; let (_ , err) = uncompress_slice (& mut output_rs , INPUT , config) ; assert_eq ! (err , ReturnCode :: DataError) ; }
};
}
