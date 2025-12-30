// Generated macro for blow_up_the_stack_2 (function)
macro_rules! Depcrate_inflateblow_up_the_stack_2 {
() => {
// Module: crate::inflate
// Provides: {"blow_up_the_stack_2"}
// Dependencies: {}
# [test] # [cfg_attr (miri , ignore = "slow")] fn blow_up_the_stack_2 () { const INPUT : & [u8] = include_bytes ! ("test-data/blow_up_the_stack_2.gz") ; let mut output_ng = vec ! [0 ; INPUT . len () * 128] ; let mut output_rs = vec ! [0 ; INPUT . len () * 128] ; let config = InflateConfig :: default () ; let (_ , err) = crate :: helpers :: uncompress_slice_ng (& mut output_ng , INPUT , config) ; assert_eq ! (err , ReturnCode :: DataError) ; let (_ , err) = uncompress_slice (& mut output_rs , INPUT , config) ; assert_eq ! (err , ReturnCode :: DataError) ; }
};
}
