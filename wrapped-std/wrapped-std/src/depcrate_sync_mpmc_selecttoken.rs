// Generated macro for Token (struct)
macro_rules! Depcrate_sync_mpmc_selectToken {
() => {
// Module: crate::sync::mpmc::select
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Temporary data that gets initialized during a blocking operation, and is consumed by"] # [doc = " `read` or `write`."] # [doc = ""] # [doc = " Each field contains data associated with a specific channel flavor."] # [derive (Debug , Default)] pub struct Token { pub (crate) array : super :: array :: ArrayToken , pub (crate) list : super :: list :: ListToken , # [allow (dead_code)] pub (crate) zero : super :: zero :: ZeroToken , }
};
}
