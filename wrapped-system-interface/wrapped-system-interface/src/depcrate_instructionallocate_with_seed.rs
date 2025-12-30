// Generated macro for allocate_with_seed (function)
macro_rules! Depcrate_instructionallocate_with_seed {
() => {
// Module: crate::instruction
// Provides: {"allocate_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn allocate_with_seed (address : & Address , base : & Address , seed : & str , space : u64 , owner : & Address ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* address , false) , AccountMeta :: new_readonly (* base , true) ,] ; Instruction :: new_with_bincode (ID , & SystemInstruction :: AllocateWithSeed { base : * base , seed : seed . to_string () , space , owner : * owner , } , account_metas ,) }
};
}
