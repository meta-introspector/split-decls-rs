// Generated macro for create_account_with_seed (function)
macro_rules! Depcrate_instructioncreate_account_with_seed {
() => {
// Module: crate::instruction
// Provides: {"create_account_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_account_with_seed (from_address : & Address , to_address : & Address , base : & Address , seed : & str , lamports : u64 , space : u64 , owner : & Address ,) -> Instruction { let mut account_metas = vec ! [AccountMeta :: new (* from_address , true) , AccountMeta :: new (* to_address , false) ,] ; if base != from_address { account_metas . push (AccountMeta :: new_readonly (* base , true)) ; } Instruction :: new_with_bincode (ID , & SystemInstruction :: CreateAccountWithSeed { base : * base , seed : seed . to_string () , lamports , space , owner : * owner , } , account_metas ,) }
};
}
