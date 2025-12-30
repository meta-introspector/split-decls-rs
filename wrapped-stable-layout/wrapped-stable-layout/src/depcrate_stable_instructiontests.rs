// Generated macro for tests (module)
macro_rules! Depcrate_stable_instructiontests {
() => {
// Module: crate::stable_instruction
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , memoffset :: offset_of , std :: mem :: { align_of , size_of } , } ; # [test] fn test_memory_layout () { assert_eq ! (offset_of ! (StableInstruction , accounts) , 0) ; assert_eq ! (offset_of ! (StableInstruction , data) , 24) ; assert_eq ! (offset_of ! (StableInstruction , program_id) , 48) ; assert_eq ! (align_of ::< StableInstruction > () , 8) ; assert_eq ! (size_of ::< StableInstruction > () , 24 + 24 + 32) ; let program_id = Pubkey :: new_unique () ; let account_meta1 = AccountMeta { pubkey : Pubkey :: new_unique () , is_signer : true , is_writable : false , } ; let account_meta2 = AccountMeta { pubkey : Pubkey :: new_unique () , is_signer : false , is_writable : true , } ; let accounts = vec ! [account_meta1 , account_meta2] ; let data = vec ! [1 , 2 , 3 , 4 , 5] ; let instruction = Instruction { program_id , accounts : accounts . clone () , data : data . clone () , } ; let instruction = StableInstruction :: from (instruction) ; let instruction_addr = & instruction as * const _ as u64 ; let accounts_ptr = instruction_addr as * const StableVec < AccountMeta > ; assert_eq ! (unsafe { &* accounts_ptr } , & accounts) ; let data_ptr = (instruction_addr + 24) as * const StableVec < u8 > ; assert_eq ! (unsafe { &* data_ptr } , & data) ; let pubkey_ptr = (instruction_addr + 48) as * const Pubkey ; assert_eq ! (unsafe { * pubkey_ptr } , program_id) ; } }
};
}
