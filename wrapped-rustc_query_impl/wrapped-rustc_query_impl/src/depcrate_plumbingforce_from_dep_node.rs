// Generated macro for force_from_dep_node (function)
macro_rules! Depcrate_plumbingforce_from_dep_node {
() => {
// Module: crate::plumbing
// Provides: {"force_from_dep_node"}
// Dependencies: {}
fn force_from_dep_node < 'tcx , Q > (query : Q , tcx : TyCtxt < 'tcx > , dep_node : DepNode) -> bool where Q : QueryConfig < QueryCtxt < 'tcx > > , { debug_assert ! (dep_node . kind != dep_kinds :: codegen_unit , "calling force_from_dep_node() on dep_kinds::codegen_unit") ; if let Some (key) = Q :: Key :: recover (tcx , & dep_node) { force_query (query , QueryCtxt :: new (tcx) , key , dep_node) ; true } else { false } }
};
}
