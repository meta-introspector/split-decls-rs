// Generated macro for SsaVisitor (struct)
macro_rules! Depcrate_ssaSsaVisitor {
() => {
// Module: crate::ssa
// Provides: {"SsaVisitor"}
// Dependencies: {}
struct SsaVisitor < 'a , 'tcx > { body : & 'a Body < 'tcx > , dominators : & 'a Dominators < BasicBlock > , assignments : IndexVec < Local , Set1 < DefLocation > > , assignment_order : Vec < Local > , direct_uses : IndexVec < Local , u32 > , borrowed_locals : DenseBitSet < Local > , }
};
}
