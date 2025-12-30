// Generated macro for impl_382 (impl)
macro_rules! Depcrate_builder_scopeimpl_382 {
() => {
// Module: crate::builder::scope
// Provides: {"impl_382"}
// Dependencies: {}
impl < 'tcx > Scopes < 'tcx > { pub (crate) fn new () -> Self { Self { scopes : Vec :: new () , breakable_scopes : Vec :: new () , const_continuable_scopes : Vec :: new () , if_then_scope : None , unwind_drops : DropTree :: new () , coroutine_drops : DropTree :: new () , } } fn push_scope (& mut self , region_scope : (region :: Scope , SourceInfo) , vis_scope : SourceScope) { debug ! ("push_scope({:?})" , region_scope) ; self . scopes . push (Scope { source_scope : vis_scope , region_scope : region_scope . 0 , drops : vec ! [] , moved_locals : vec ! [] , cached_unwind_block : None , cached_coroutine_drop_block : None , }) ; } fn pop_scope (& mut self , region_scope : (region :: Scope , SourceInfo)) -> Scope { let scope = self . scopes . pop () . unwrap () ; assert_eq ! (scope . region_scope , region_scope . 0) ; scope } fn scope_index (& self , region_scope : region :: Scope , span : Span) -> usize { self . scopes . iter () . rposition (| scope | scope . region_scope == region_scope) . unwrap_or_else (| | span_bug ! (span , "region_scope {:?} does not enclose" , region_scope)) } # [doc = " Returns the topmost active scope, which is known to be alive until"] # [doc = " the next scope expression."] fn topmost (& self) -> region :: Scope { self . scopes . last () . expect ("topmost_scope: no scopes present") . region_scope } }
};
}
