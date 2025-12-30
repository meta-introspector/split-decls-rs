// Generated macro for QueryOriginKind (enum)
macro_rules! Depcrate_zalsa_localQueryOriginKind {
() => {
// Module: crate::zalsa_local
// Provides: {"QueryOriginKind"}
// Dependencies: {}
# [derive (Clone , Copy)] # [repr (u8)] enum QueryOriginKind { # [doc = " An initial provisional value."] # [doc = ""] # [doc = " This will occur occur in queries that support fixpoint iteration."] FixpointInitial = 0b00 , # [doc = " The value was assigned as the output of another query."] # [doc = ""] # [doc = " This can, for example, can occur when `specify` is used."] Assigned = 0b01 , # [doc = " The value was derived by executing a function"] # [doc = " _and_ Salsa was able to track all of said function's inputs."] Derived = 0b11 , # [doc = " The value was derived by executing a function"] # [doc = " but that function also reported that it read untracked inputs."] DerivedUntracked = 0b10 , }
};
}
