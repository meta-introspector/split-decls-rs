// Generated macro for assign_with_seed (function)
macro_rules! Depcrate_instructionassign_with_seed {
() => {
// Module: crate::instruction
// Provides: {"assign_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn assign_with_seed (address : & Address , base : & Address , seed : & str , owner : & Address ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* address , false) , AccountMeta :: new_readonly (* base , true) ,] ; Instruction :: new_with_bincode (ID , & SystemInstruction :: AssignWithSeed { base : * base , seed : seed . to_string () , owner : * owner , } , account_metas ,) }
};
}
