// Generated macro for TmpFlags (struct)
macro_rules! Depcrate_tmpTmpFlags {
() => {
// Module: crate::tmp
// Provides: {"TmpFlags"}
// Dependencies: {}
# [doc = " Temporary data that is automatically resized."] # [doc = ""] # [doc = " This contains buffers that are automatically resized when the variable count of the solver"] # [doc = " changes. They are also always kept in a clean state, so using them doesn't come with costs"] # [doc = " proportional to the number of variables."] # [doc = ""] # [doc = " Make sure to check any documented invariants when using this. Also make sure to check all"] # [doc = " existing users when adding invariants."] # [derive (Default)] pub struct TmpFlags { # [doc = " A boolean for each literal."] # [doc = ""] # [doc = " Reset to all-false, keep size."] pub flags : Vec < bool > , }
};
}
