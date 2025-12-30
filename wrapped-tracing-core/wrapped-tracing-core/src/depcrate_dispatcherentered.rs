// Generated macro for Entered (struct)
macro_rules! Depcrate_dispatcherEntered {
() => {
// Module: crate::dispatcher
// Provides: {"Entered"}
// Dependencies: {}
# [doc = " While this guard is active, additional calls to subscriber functions on"] # [doc = " the default dispatcher will not be able to access the dispatch context."] # [doc = " Dropping the guard will allow the dispatch context to be re-entered."] # [cfg (feature = "std")] struct Entered < 'a > (& 'a State) ;
};
}
