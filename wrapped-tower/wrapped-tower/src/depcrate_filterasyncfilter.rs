// Generated macro for AsyncFilter (struct)
macro_rules! Depcrate_filterAsyncFilter {
() => {
// Module: crate::filter
// Provides: {"AsyncFilter"}
// Dependencies: {}
# [doc = " Conditionally dispatch requests to the inner service based on an"] # [doc = " [asynchronous predicate]."] # [doc = ""] # [doc = " [asynchronous predicate]: AsyncPredicate"] # [derive (Clone , Debug)] pub struct AsyncFilter < T , U > { inner : T , predicate : U , }
};
}
