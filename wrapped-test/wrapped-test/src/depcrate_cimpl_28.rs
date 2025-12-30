// Generated macro for impl_28 (impl)
macro_rules! Depcrate_cimpl_28 {
() => {
// Module: crate::c
// Provides: {"impl_28"}
// Dependencies: {}
impl LanguageMethods for C { fn display (& self) -> & str { "c" } fn comment_prefix_for_test_config (& self) -> Option < & str > { Some ("//@") } fn should_fail_verify (& self , _name : & str , config : & crate :: config :: WitConfig , _args : & [String] ,) -> bool { config . error_context } fn codegen_test_variants (& self) -> & [(& str , & [& str])] { & [("no-sig-flattening" , & ["--no-sig-flattening"]) , ("autodrop" , & ["--autodrop-borrows=yes"]) , ("async" , & ["--async=all"]) ,] } fn prepare (& self , runner : & mut Runner < '_ >) -> Result < () > { prepare (runner , clang (runner)) } fn compile (& self , runner : & Runner < '_ > , c : & Compile < '_ >) -> Result < () > { compile (runner , c , clang (runner)) } fn verify (& self , runner : & Runner < '_ > , v : & Verify < '_ >) -> Result < () > { verify (runner , v , clang (runner)) } }
};
}
