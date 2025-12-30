// Generated macro for empty_visit_mut (macro)
macro_rules! Depcrate_visit_mutempty_visit_mut {
() => {
// Module: crate::visit_mut
// Provides: {"empty_visit_mut"}
// Dependencies: {}
macro_rules ! empty_visit_mut { ($ name : ident , $ t : ty) => { fn $ name < V > (_v : & mut V , _node : & mut $ t) where V : VisitMut + ? Sized , { } } ; }
};
}
