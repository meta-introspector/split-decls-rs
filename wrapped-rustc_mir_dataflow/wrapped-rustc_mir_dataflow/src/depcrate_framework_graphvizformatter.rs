// Generated macro for Formatter (struct)
macro_rules! Depcrate_framework_graphvizFormatter {
() => {
// Module: crate::framework::graphviz
// Provides: {"Formatter"}
// Dependencies: {}
struct Formatter < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { body : & 'mir Body < 'tcx > , analysis : RefCell < & 'mir mut A > , results : & 'mir Results < A :: Domain > , style : OutputStyle , reachable : DenseBitSet < BasicBlock > , }
};
}
