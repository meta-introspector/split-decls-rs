// Generated macro for impl_42 (impl)
macro_rules! Depcrate_collectorimpl_42 {
() => {
// Module: crate::collector
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'tcx > MonoItems < 'tcx > { fn new () -> Self { Self { items : FxIndexMap :: default () } } fn is_empty (& self) -> bool { self . items . is_empty () } fn push (& mut self , item : Spanned < MonoItem < 'tcx > >) { self . items . entry (item . node) . or_insert (item . span) ; } fn items (& self) -> impl Iterator < Item = MonoItem < 'tcx > > { self . items . keys () . cloned () } }
};
}
