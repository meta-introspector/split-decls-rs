// Generated macro for macro_760 (macro)
macro_rules! Depcrate_util_call_all_commonmacro_760 {
() => {
// Module: crate::util::call_all::common
// Provides: {"macro_760"}
// Dependencies: {}
pin_project ! { # [doc = " The [`Future`] returned by the [`ServiceExt::call_all`] combinator."] pub (crate) struct CallAll < Svc , S , Q > where S : Stream , { service : Option < Svc >, # [pin] stream : S , queue : Q , eof : bool , curr_req : Option < S :: Item > } }
};
}
