// Generated macro for visit_array_mut (function)
macro_rules! Depcrate_visit_mutvisit_array_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_array_mut"}
// Dependencies: {}
pub fn visit_array_mut < V > (v : & mut V , node : & mut Array) where V : VisitMut + ? Sized , { for value in node . iter_mut () { v . visit_value_mut (value) ; } }
};
}
