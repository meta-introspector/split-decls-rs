// Generated macro for NoClientAuth (struct)
macro_rules! Depcrate_verifyNoClientAuth {
() => {
// Module: crate::verify
// Provides: {"NoClientAuth"}
// Dependencies: {}
# [doc = " Turns off client authentication."] # [doc = ""] # [doc = " In contrast to using"] # [doc = " `WebPkiClientVerifier::builder(roots).allow_unauthenticated().build()`, the `NoClientAuth`"] # [doc = " `ClientVerifier` will not offer client authentication at all, vs offering but not"] # [doc = " requiring it."] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct NoClientAuth ;
};
}
