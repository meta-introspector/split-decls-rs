// Generated macro for transfer_with_seed (function)
macro_rules! Depcrate_instructiontransfer_with_seed {
() => {
// Module: crate::instruction
// Provides: {"transfer_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn transfer_with_seed (from_address : & Address , from_base : & Address , from_seed : String , from_owner : & Address , to_address : & Address , lamports : u64 ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* from_address , false) , AccountMeta :: new_readonly (* from_base , true) , AccountMeta :: new (* to_address , false) ,] ; Instruction :: new_with_bincode (ID , & SystemInstruction :: TransferWithSeed { lamports , from_seed , from_owner : * from_owner , } , account_metas ,) }
};
}
