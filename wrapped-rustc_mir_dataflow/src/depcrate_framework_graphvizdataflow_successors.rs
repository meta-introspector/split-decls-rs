// Generated macro for dataflow_successors (function)
macro_rules! Depcrate_framework_graphvizdataflow_successors {
() => {
// Module: crate::framework::graphviz
// Provides: {"dataflow_successors"}
// Dependencies: {}
fn dataflow_successors (body : & Body < '_ > , bb : BasicBlock) -> Vec < CfgEdge > { body [bb] . terminator () . successors () . enumerate () . map (| (index , _) | CfgEdge { source : bb , index }) . collect () }
};
}
