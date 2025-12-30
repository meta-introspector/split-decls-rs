// Generated macro for copy_into (function)
macro_rules! Depcrate_headercopy_into {
() => {
// Module: crate::header
// Provides: {"copy_into"}
// Dependencies: {}
# [doc = " Copies `bytes` into the `slot` provided, returning an error if the `bytes`"] # [doc = " array is too long or if it contains any nul bytes."] fn copy_into (slot : & mut [u8] , bytes : & [u8]) -> io :: Result < () > { if bytes . len () > slot . len () { Err (other ("provided value is too long")) } else if bytes . contains (& 0) { Err (other ("provided value contains a nul byte")) } else { for (slot , val) in slot . iter_mut () . zip (bytes . iter () . chain (Some (& 0))) { * slot = * val ; } Ok (()) } }
};
}
