// Generated macro for DefIdVisitorSkeleton (struct)
macro_rules! DepcrateDefIdVisitorSkeleton {
() => {
// Module: crate
// Provides: {"DefIdVisitorSkeleton"}
// Dependencies: {}
pub struct DefIdVisitorSkeleton < 'v , 'tcx , V : ? Sized > { def_id_visitor : & 'v mut V , visited_opaque_tys : FxHashSet < DefId > , dummy : PhantomData < TyCtxt < 'tcx > > , }
};
}
