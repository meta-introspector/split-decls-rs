// Generated macro for assert_tokens (function)
macro_rules! Depcrateassert_tokens {
() => {
// Module: crate
// Provides: {"assert_tokens"}
// Dependencies: {}
# [doc = "\nAssert that a value streams to exactly the sequence of tokens provided.\n"] # [track_caller] pub fn assert_tokens < 'sval , V : sval :: Value + ? Sized > (value : & 'sval V , tokens : & [Token < 'sval >]) { let mut stream = TokenBuf :: new () ; match value . stream (& mut stream) { Ok (()) => { assert_eq ! (tokens , stream . as_tokens () , "{} != {}" , sval_fmt :: stream_to_string (AsValue (tokens)) , sval_fmt :: stream_to_string (AsValue (stream . as_tokens ()))) ; # [cfg (test)] { let mut dyn_stream = & mut TokenBuf :: new () ; value . stream (& mut dyn_stream as & mut dyn sval_dynamic :: Stream < 'sval >) . unwrap () ; assert_eq ! (tokens , dyn_stream . as_tokens () , "(dyn) {} != {}" , sval_fmt :: stream_to_string (AsValue (tokens)) , sval_fmt :: stream_to_string (AsValue (dyn_stream . as_tokens ()))) ; } } Err (_) => stream . fail :: < V > () , } }
};
}
