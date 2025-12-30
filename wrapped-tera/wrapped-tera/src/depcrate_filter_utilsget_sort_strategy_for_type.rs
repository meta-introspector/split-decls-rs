// Generated macro for get_sort_strategy_for_type (function)
macro_rules! Depcrate_filter_utilsget_sort_strategy_for_type {
() => {
// Module: crate::filter_utils
// Provides: {"get_sort_strategy_for_type"}
// Dependencies: {}
pub fn get_sort_strategy_for_type (ty : & Value) -> Result < Box < dyn SortStrategy > > { use crate :: Value :: * ; match * ty { Null => Err (Error :: msg ("Null is not a sortable value")) , Bool (_) => Ok (Box :: < SortBools > :: default ()) , Number (_) => Ok (Box :: < SortNumbers > :: default ()) , String (_) => Ok (Box :: < SortStrings > :: default ()) , Array (_) => Ok (Box :: < SortArrays > :: default ()) , Object (_) => Err (Error :: msg ("Object is not a sortable value")) , } }
};
}
