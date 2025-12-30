// Generated macro for create_loadable_account_with_fields (function)
macro_rules! Depcrate_native_loadercreate_loadable_account_with_fields {
() => {
// Module: crate::native_loader
// Provides: {"create_loadable_account_with_fields"}
// Dependencies: {}
# [doc = " Create an executable account with the given shared object name."] pub fn create_loadable_account_with_fields (name : & str , (lamports , rent_epoch) : InheritableAccountFields ,) -> AccountSharedData { AccountSharedData :: from (Account { lamports , owner : id () , data : name . as_bytes () . to_vec () , executable : true , rent_epoch , }) }
};
}
