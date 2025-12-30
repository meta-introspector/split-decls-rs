// Generated macro for impl_146 (impl)
macro_rules! Depcrate_debugger_visualizerimpl_146 {
() => {
// Module: crate::debugger_visualizer
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for DebuggerVisualizerCollector < '_ > { fn visit_attribute (& mut self , attr : & 'ast Attribute) { self . check_for_debugger_visualizer (attr) ; rustc_ast :: visit :: walk_attribute (self , attr) ; } }
};
}
