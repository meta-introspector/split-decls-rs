// Generated macro for ZeroTrieBuildError (enum)
macro_rules! Depcrate_errorZeroTrieBuildError {
() => {
// Module: crate::error
// Provides: {"ZeroTrieBuildError"}
// Dependencies: {}
# [doc = " Error types for the `zerotrie` crate."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Display)] # [non_exhaustive] pub enum ZeroTrieBuildError { # [doc = " Non-ASCII data was added to an ASCII-only trie."] # [displaydoc ("Non-ASCII cannot be added to an ASCII-only trie")] NonAsciiError , # [doc = " The trie reached its maximum supported capacity."] # [displaydoc ("Reached maximum capacity of trie")] CapacityExceeded , # [doc = " The builder could not solve the perfect hash function."] # [displaydoc ("Failed to solve the perfect hash function. This is rare! Please report your case to the ICU4X team.")] CouldNotSolvePerfectHash , # [doc = " Mixed-case data was added to a case-insensitive trie."] # [displaydoc ("Mixed-case data added to case-insensitive trie")] MixedCase , }
};
}
