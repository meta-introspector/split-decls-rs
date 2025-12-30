// Generated macro for debugger_visualizers (function)
macro_rules! Depcrate_debugger_visualizerdebugger_visualizers {
() => {
// Module: crate::debugger_visualizer
// Provides: {"debugger_visualizers"}
// Dependencies: {}
# [doc = " Traverses and collects the debugger visualizers for a specific crate."] fn debugger_visualizers (tcx : TyCtxt < '_ > , _ : LocalCrate) -> Vec < DebuggerVisualizerFile > { let resolver_and_krate = tcx . resolver_for_lowering () . borrow () ; let krate = & * resolver_and_krate . 1 ; let mut visitor = DebuggerVisualizerCollector { sess : tcx . sess , visualizers : Vec :: new () } ; rustc_ast :: visit :: Visitor :: visit_crate (& mut visitor , krate) ; visitor . visualizers }
};
}
