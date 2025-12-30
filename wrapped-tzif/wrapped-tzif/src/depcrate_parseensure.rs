// Generated macro for ensure (function)
macro_rules! Depcrate_parseensure {
() => {
// Module: crate::parse
// Provides: {"ensure"}
// Dependencies: {}
# [doc = " Ensures that the predicate is [`true`], otherwise returns an error with the provided"] # [doc = " messages though combine's error machinery."] fn ensure < Input : Stream , L > (output : L , predicate : impl FnOnce (& L) -> bool , message : & 'static str ,) -> Either < Value < Input , L > , Unexpected < Input , L , & 'static str > > where L : Clone , { if predicate (& output) { value (output) . left () } else { Parser :: right (unexpected_any (message)) } }
};
}
