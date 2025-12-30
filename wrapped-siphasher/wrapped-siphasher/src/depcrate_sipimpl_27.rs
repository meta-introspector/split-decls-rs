// Generated macro for impl_27 (impl)
macro_rules! Depcrate_sipimpl_27 {
() => {
// Module: crate::sip
// Provides: {"impl_27"}
// Dependencies: {}
impl < S : Sip > Default for Hasher < S > { # [doc = " Creates a `Hasher<S>` with the two initial keys set to 0."] # [inline] fn default () -> Hasher < S > { Hasher :: new_with_keys (0 , 0) } }
};
}
