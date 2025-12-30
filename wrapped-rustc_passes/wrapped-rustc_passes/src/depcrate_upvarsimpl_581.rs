// Generated macro for impl_581 (impl)
macro_rules! Depcrate_upvarsimpl_581 {
() => {
// Module: crate::upvars
// Provides: {"impl_581"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for LocalCollector { fn visit_pat (& mut self , pat : & 'tcx hir :: Pat < 'tcx >) { if let hir :: PatKind :: Binding (_ , hir_id , ..) = pat . kind { self . locals . insert (hir_id) ; } intravisit :: walk_pat (self , pat) ; } }
};
}
