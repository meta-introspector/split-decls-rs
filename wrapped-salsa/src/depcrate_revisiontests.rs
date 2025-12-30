// Generated macro for tests (module)
macro_rules! Depcrate_revisiontests {
() => {
// Module: crate::revision
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn optional_atomic_revision () { let val = OptionalAtomicRevision :: new (Some (Revision :: start ())) ; assert_eq ! (val . load () , Some (Revision :: start ())) ; assert_eq ! (val . swap (None) , Some (Revision :: start ())) ; assert_eq ! (val . load () , None) ; assert_eq ! (val . swap (Some (Revision :: start ())) , None) ; assert_eq ! (val . load () , Some (Revision :: start ())) ; assert_eq ! (val . compare_exchange (Some (Revision :: start ()) , None) , Ok (Some (Revision :: start ()))) ; assert_eq ! (val . compare_exchange (Some (Revision :: start ()) , None) , Err (None)) ; } }
};
}
