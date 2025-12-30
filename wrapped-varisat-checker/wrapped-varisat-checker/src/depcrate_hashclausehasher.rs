// Generated macro for ClauseHasher (struct)
macro_rules! Depcrate_hashClauseHasher {
() => {
// Module: crate::hash
// Provides: {"ClauseHasher"}
// Dependencies: {}
pub struct ClauseHasher { # [doc = " How many bits are used for storing clause hashes."] pub hash_bits : u32 , # [doc = " Changed solver names that are not yet reflected in the checkers current clause hashes."] pub buffered_solver_var_names : Vec < (Var , Option < Var >) > , # [doc = " Does buffered_solver_var_names contain a new name?"] # [doc = ""] # [doc = " If it contains only deletes, there is no need to rehash"] pub rename_in_buffered_solver_var_names : bool , # [doc = " Current mapping from global var names to solver var names, used for hashing."] solver_var_names : HashMap < Var , Var > , }
};
}
