// Generated macro for Context (struct)
macro_rules! Depcrate_dictionary_coverContext {
() => {
// Module: crate::dictionary::cover
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A re-usable allocation containing large allocations"] # [doc = " that are used multiple times during dictionary construction (once per epoch)"] pub struct Context { # [doc = " Keeps track of the number of occurances of a particular k-mer within an epoch."] # [doc = ""] # [doc = " Reset for each epoch."] pub frequencies : HashMap < KMer , usize > , }
};
}
