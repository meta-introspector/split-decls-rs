// Generated macro for model_complete_take_while_m_n (function)
macro_rules! Depcrate_token_testsmodel_complete_take_while_m_n {
() => {
// Module: crate::token::tests
// Provides: {"model_complete_take_while_m_n"}
// Dependencies: {}
# [cfg (feature = "std")] fn model_complete_take_while_m_n < 'i > (m : usize , n : usize , valid : usize , input : & mut & 'i str ,) -> ModalResult < & 'i str > { if n < m { Err (crate :: error :: ParserError :: from_input (input)) } else if m <= valid { let offset = n . min (valid) ; Ok (input . next_slice (offset)) } else { Err (crate :: error :: ParserError :: from_input (input)) } }
};
}
