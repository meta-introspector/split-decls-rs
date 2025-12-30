// Generated macro for LiveNodeKind (enum)
macro_rules! Depcrate_livenessLiveNodeKind {
() => {
// Module: crate::liveness
// Provides: {"LiveNodeKind"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Debug)] enum LiveNodeKind { UpvarNode (Span) , ExprNode (Span , HirId) , VarDefNode (Span , HirId) , ClosureNode , ExitNode , }
};
}
