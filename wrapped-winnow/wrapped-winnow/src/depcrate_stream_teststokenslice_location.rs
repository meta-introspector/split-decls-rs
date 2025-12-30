// Generated macro for tokenslice_location (function)
macro_rules! Depcrate_stream_teststokenslice_location {
() => {
// Module: crate::stream::tests
// Provides: {"tokenslice_location"}
// Dependencies: {}
# [test] fn tokenslice_location () { # [derive (Clone , Debug)] struct Token { span : core :: ops :: Range < usize > , } impl Location for Token { # [inline (always)] fn previous_token_end (& self) -> usize { self . span . end } # [inline (always)] fn current_token_start (& self) -> usize { self . span . start } } let input = [Token { span : 1 .. 9 } , Token { span : 11 .. 19 } , Token { span : 21 .. 29 } ,] ; let mut input = TokenSlice :: new (& input) ; assert_eq ! (input . previous_token_end () , 1) ; assert_eq ! (input . current_token_start () , 1) ; let _ = input . next_token () ; assert_eq ! (input . previous_token_end () , 9) ; assert_eq ! (input . current_token_start () , 11) ; let _ = input . next_token () ; assert_eq ! (input . previous_token_end () , 19) ; assert_eq ! (input . current_token_start () , 21) ; let _ = input . next_token () ; assert_eq ! (input . previous_token_end () , 29) ; assert_eq ! (input . current_token_start () , 29) ; }
};
}
