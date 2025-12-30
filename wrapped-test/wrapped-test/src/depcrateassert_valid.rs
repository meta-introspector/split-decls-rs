// Generated macro for assert_valid (function)
macro_rules! Depcrateassert_valid {
() => {
// Module: crate
// Provides: {"assert_valid"}
// Dependencies: {}
# [doc = "\nAssert that a value streams without failing.\n"] # [track_caller] pub fn assert_valid < V : sval :: Value > (value : V) { let mut stream = TokenBuf :: new () ; if let Err (_) = value . stream (& mut stream) { stream . fail :: < V > () ; } }
};
}
