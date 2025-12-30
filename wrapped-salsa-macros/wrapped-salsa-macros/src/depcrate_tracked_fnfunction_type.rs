// Generated macro for function_type (function)
macro_rules! Depcrate_tracked_fnfunction_type {
() => {
// Module: crate::tracked_fn
// Provides: {"function_type"}
// Dependencies: {}
fn function_type (item_fn : & syn :: ItemFn) -> FunctionType { match item_fn . sig . inputs . len () { 0 => unreachable ! ("functions have been checked to have at least a database argument by this point") , 1 => FunctionType :: Constant , 2 => FunctionType :: SalsaStruct , _ => FunctionType :: RequiresInterning , } }
};
}
