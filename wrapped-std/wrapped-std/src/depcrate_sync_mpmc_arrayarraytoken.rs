// Generated macro for ArrayToken (struct)
macro_rules! Depcrate_sync_mpmc_arrayArrayToken {
() => {
// Module: crate::sync::mpmc::array
// Provides: {"ArrayToken"}
// Dependencies: {}
# [doc = " The token type for the array flavor."] # [derive (Debug)] pub (crate) struct ArrayToken { # [doc = " Slot to read from or write to."] slot : * const u8 , # [doc = " Stamp to store into the slot after reading or writing."] stamp : usize , }
};
}
