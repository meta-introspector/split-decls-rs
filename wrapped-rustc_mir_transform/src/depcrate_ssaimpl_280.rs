// Generated macro for impl_280 (impl)
macro_rules! Depcrate_ssaimpl_280 {
() => {
// Module: crate::ssa
// Provides: {"impl_280"}
// Dependencies: {}
impl SsaVisitor < '_ , '_ > { fn check_dominates (& mut self , local : Local , loc : Location) { let set = & mut self . assignments [local] ; let assign_dominates = match * set { Set1 :: Empty | Set1 :: Many => false , Set1 :: One (def) => def . dominates (loc , self . dominators) , } ; if ! assign_dominates { * set = Set1 :: Many ; } } }
};
}
