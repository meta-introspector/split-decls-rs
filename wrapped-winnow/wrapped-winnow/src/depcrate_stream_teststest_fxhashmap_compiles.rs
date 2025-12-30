// Generated macro for test_fxhashmap_compiles (function)
macro_rules! Depcrate_stream_teststest_fxhashmap_compiles {
() => {
// Module: crate::stream::tests
// Provides: {"test_fxhashmap_compiles"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] fn test_fxhashmap_compiles () { let input = "a=b" ; fn pair (i : & mut & str) -> ModalResult < (char , char) > { let out = separated_pair ('a' , '=' , 'b') . parse_next (i) ? ; Ok (out) } let _ : rustc_hash :: FxHashMap < char , char > = separated (0 .. , pair , ',') . parse (input) . unwrap () ; }
};
}
