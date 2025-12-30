// Generated macro for optimized_mir (function)
macro_rules! Depcrateoptimized_mir {
() => {
// Module: crate
// Provides: {"optimized_mir"}
// Dependencies: {}
# [doc = " Optimize the MIR and prepare it for codegen."] fn optimized_mir (tcx : TyCtxt < '_ > , did : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_optimized_mir (tcx , did)) }
};
}
