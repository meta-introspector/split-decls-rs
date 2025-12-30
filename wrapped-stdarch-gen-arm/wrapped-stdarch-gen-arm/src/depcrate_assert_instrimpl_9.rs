// Generated macro for impl_9 (impl)
macro_rules! Depcrate_assert_instrimpl_9 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_9"}
// Dependencies: {}
impl InstructionAssertion { fn build (& mut self , ctx : & Context) -> context :: Result { match self { InstructionAssertion :: Basic (ws) => ws . build_acle (ctx . local) , InstructionAssertion :: WithArgs (ws , args_ws) => [ws , args_ws] . into_iter () . try_for_each (| ws | ws . build_acle (ctx . local)) , } } }
};
}
