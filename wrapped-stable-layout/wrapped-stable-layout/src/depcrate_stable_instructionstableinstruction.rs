// Generated macro for StableInstruction (struct)
macro_rules! Depcrate_stable_instructionStableInstruction {
() => {
// Module: crate::stable_instruction
// Provides: {"StableInstruction"}
// Dependencies: {}
# [doc = " `Instruction`, with a stable memory layout"] # [doc = ""] # [doc = " This is used within the runtime to ensure memory mapping and memory accesses are valid.  We"] # [doc = " rely on known addresses and offsets within the runtime, and since `Instruction`'s layout is"] # [doc = " allowed to change, we must provide a way to lock down the memory layout.  `StableInstruction`"] # [doc = " reimplements the bare minimum of `Instruction`'s API sufficient only for the runtime's needs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creating a `StableInstruction` from an `Instruction`"] # [doc = ""] # [doc = " ```"] # [doc = " # use solana_instruction::Instruction;"] # [doc = " # use solana_pubkey::Pubkey;"] # [doc = " # use solana_stable_layout::stable_instruction::StableInstruction;"] # [doc = " # let program_id = Pubkey::default();"] # [doc = " # let accounts = Vec::default();"] # [doc = " # let data = Vec::default();"] # [doc = " let instruction = Instruction { program_id, accounts, data };"] # [doc = " let instruction = StableInstruction::from(instruction);"] # [doc = " ```"] # [derive (Debug , PartialEq)] # [repr (C)] pub struct StableInstruction { pub accounts : StableVec < AccountMeta > , pub data : StableVec < u8 > , pub program_id : Pubkey , }
};
}
