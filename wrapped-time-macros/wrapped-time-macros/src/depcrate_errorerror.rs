// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
pub (crate) enum Error { MissingComponent { name : & 'static str , span_start : Option < Span > , span_end : Option < Span > , } , InvalidComponent { name : & 'static str , value : String , span_start : Option < Span > , span_end : Option < Span > , } , # [cfg (any (feature = "formatting" , feature = "parsing"))] ExpectedString { span_start : Option < Span > , span_end : Option < Span > , } , UnexpectedToken { tree : TokenTree , } , UnexpectedEndOfInput , Custom { message : Cow < 'static , str > , span_start : Option < Span > , span_end : Option < Span > , } , }
};
}
