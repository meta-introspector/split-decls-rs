// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " This function has been renamed to `block_on`; please see its documentation."] # [doc = " This function remains to maintain compatibility with the online versions"] # [doc = " of the book that use the name `run`."] pub fn run < F : Future > (future : F) -> F :: Output { block_on (future) }
};
}
