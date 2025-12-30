// Generated macro for assert_invalid (function)
macro_rules! Depcrateassert_invalid {
() => {
// Module: crate
// Provides: {"assert_invalid"}
// Dependencies: {}
# [doc = "\nAssert that a value fails to stream.\n"] # [track_caller] pub fn assert_invalid < V : sval :: Value > (value : V) { let mut stream = TokenBuf :: new () ; if let Ok (_) = value . stream (& mut stream) { panic ! ("expected streaming `{}` to fail, but it produced `{}`" , type_name ::< V > () , sval_fmt :: stream_to_string (AsValue (& stream . tokens))) } }
};
}
