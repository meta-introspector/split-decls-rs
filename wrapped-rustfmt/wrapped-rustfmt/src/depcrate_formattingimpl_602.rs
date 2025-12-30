// Generated macro for impl_602 (impl)
macro_rules! Depcrate_formattingimpl_602 {
() => {
// Module: crate::formatting
// Provides: {"impl_602"}
// Dependencies: {}
impl < 'b , T : Write + 'b > Session < 'b , T > { pub (crate) fn format_input_inner (& mut self , input : Input , is_macro_def : bool ,) -> Result < FormatReport , ErrorKind > { if ! self . config . version_meets_requirement () { return Err (ErrorKind :: VersionMismatch) ; } rustc_span :: create_session_if_not_set_then (self . config . edition () . into () , | _ | { if self . config . disable_all_formatting () { return match input { Input :: Text (ref buf) => echo_back_stdin (buf) , _ => Ok (FormatReport :: new ()) , } ; } let config = & self . config . clone () ; let format_result = format_project (input , config , self , is_macro_def) ; format_result . map (| report | { self . errors . add (& report . internal . borrow () . 1) ; report }) }) } }
};
}
