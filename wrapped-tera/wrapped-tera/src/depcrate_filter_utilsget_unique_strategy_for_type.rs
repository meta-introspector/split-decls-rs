// Generated macro for get_unique_strategy_for_type (function)
macro_rules! Depcrate_filter_utilsget_unique_strategy_for_type {
() => {
// Module: crate::filter_utils
// Provides: {"get_unique_strategy_for_type"}
// Dependencies: {}
pub fn get_unique_strategy_for_type (ty : & Value , case_sensitive : bool ,) -> Result < Box < dyn UniqueStrategy > > { use crate :: Value :: * ; match * ty { Null => Err (Error :: msg ("Null is not a unique value")) , Bool (_) => Ok (Box :: < UniqueBools > :: default ()) , Number (ref val) => { if val . is_f64 () { Err (Error :: msg ("Unique floats are not implemented")) } else { Ok (Box :: < UniqueNumbers > :: default ()) } } String (_) => Ok (Box :: new (UniqueStrings :: new (case_sensitive))) , Array (_) => Err (Error :: msg ("Unique arrays are not implemented")) , Object (_) => Err (Error :: msg ("Unique objects are not implemented")) , } }
};
}
