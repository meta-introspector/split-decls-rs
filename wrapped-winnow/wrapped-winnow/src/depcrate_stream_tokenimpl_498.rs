// Generated macro for impl_498 (impl)
macro_rules! Depcrate_stream_tokenimpl_498 {
() => {
// Module: crate::stream::token
// Provides: {"impl_498"}
// Dependencies: {}
# [doc = " Track locations by implementing [`Location`] on the Token."] impl < T > TokenSlice < '_ , T > where T : Location , { # [inline (always)] fn previous_token_end (& self) -> Option < usize > { let index = self . input . offset_from (& self . initial) ; index . checked_sub (1) . map (| i | self . initial [i] . previous_token_end ()) } # [inline (always)] fn current_token_start (& self) -> Option < usize > { self . input . first () . map (| t | t . current_token_start ()) } }
};
}
