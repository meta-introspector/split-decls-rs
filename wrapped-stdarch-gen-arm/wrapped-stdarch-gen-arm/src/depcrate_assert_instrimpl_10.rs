// Generated macro for impl_10 (impl)
macro_rules! Depcrate_assert_instrimpl_10 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_10"}
// Dependencies: {}
impl ToTokens for InstructionAssertion { fn to_tokens (& self , tokens : & mut TokenStream) { let instr = format_ident ! ("{}" , match self { Self :: Basic (instr) => instr , Self :: WithArgs (instr , _) => instr , } . to_string ()) ; tokens . append_all (quote ! { # instr }) ; if let Self :: WithArgs (_ , args) = self { let ex : TokenStream = args . to_string () . parse () . expect ("invalid instruction assertion arguments expression given") ; tokens . append_all (quote ! { , # ex }) } } }
};
}
