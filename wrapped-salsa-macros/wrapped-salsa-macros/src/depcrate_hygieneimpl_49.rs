// Generated macro for impl_49 (impl)
macro_rules! Depcrate_hygieneimpl_49 {
() => {
// Module: crate::hygiene
// Provides: {"impl_49"}
// Dependencies: {}
impl Hygiene { pub fn from1 (tokens : & proc_macro :: TokenStream) -> Self { let mut user_tokens = HashSet :: new () ; push_idents1 (tokens . clone () , & mut user_tokens) ; Self { user_tokens } } pub fn from2 (tokens : & impl ToTokens) -> Self { let mut user_tokens = HashSet :: new () ; push_idents2 (tokens . to_token_stream () , & mut user_tokens) ; Self { user_tokens } } }
};
}
