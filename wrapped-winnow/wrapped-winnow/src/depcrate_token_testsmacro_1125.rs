// Generated macro for macro_1125 (macro)
macro_rules! Depcrate_token_testsmacro_1125 {
() => {
// Module: crate::token::tests
// Provides: {"macro_1125"}
// Dependencies: {}
# [cfg (feature = "std")] proptest ! { # [test] # [cfg_attr (miri , ignore)] fn complete_take_while_m_n_bounds (m in 0 .. 20usize , n in 0 .. 20usize , valid in 0 .. 20usize , invalid in 0 .. 20usize) { let input = format ! ("{:a<valid$}{:b<invalid$}" , "" , "" , valid = valid , invalid = invalid) ; let mut model_input = input . as_str () ; let expected = model_complete_take_while_m_n (m , n , valid , & mut model_input) ; if m <= n { let actual = take_while (m ..= n , | c : char | c == 'a') . parse_peek (input . as_str ()) ; assert_eq ! (expected . map (| o | (model_input , o)) , actual) ; } } }
};
}
