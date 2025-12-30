// Generated macro for impl_4 (impl)
macro_rules! Depcrate_stable_instructionimpl_4 {
() => {
// Module: crate::stable_instruction
// Provides: {"impl_4"}
// Dependencies: {}
impl From < Instruction > for StableInstruction { fn from (other : Instruction) -> Self { Self { accounts : other . accounts . into () , data : other . data . into () , program_id : other . program_id , } } }
};
}
