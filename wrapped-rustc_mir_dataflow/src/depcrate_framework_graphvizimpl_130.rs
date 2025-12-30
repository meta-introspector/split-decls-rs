// Generated macro for impl_130 (impl)
macro_rules! Depcrate_framework_graphvizimpl_130 {
() => {
// Module: crate::framework::graphviz
// Provides: {"impl_130"}
// Dependencies: {}
impl < D > StateDiffCollector < D > { fn run < 'tcx , A > (body : & Body < 'tcx > , block : BasicBlock , analysis : & mut A , results : & Results < A :: Domain > , style : OutputStyle ,) -> Self where A : Analysis < 'tcx , Domain = D > , D : DebugWithContext < A > , { let mut collector = StateDiffCollector { prev_state : analysis . bottom_value (body) , after : vec ! [] , before : (style == OutputStyle :: BeforeAndAfter) . then_some (vec ! []) , } ; visit_results (body , std :: iter :: once (block) , analysis , results , & mut collector) ; collector } }
};
}
