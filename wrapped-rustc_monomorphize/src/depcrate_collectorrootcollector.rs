// Generated macro for RootCollector (struct)
macro_rules! Depcrate_collectorRootCollector {
() => {
// Module: crate::collector
// Provides: {"RootCollector"}
// Dependencies: {}
struct RootCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , strategy : MonoItemCollectionStrategy , output : & 'a mut MonoItems < 'tcx > , entry_fn : Option < (DefId , EntryFnType) > , }
};
}
