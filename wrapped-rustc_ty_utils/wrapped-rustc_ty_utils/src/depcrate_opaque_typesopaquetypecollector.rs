// Generated macro for OpaqueTypeCollector (struct)
macro_rules! Depcrate_opaque_typesOpaqueTypeCollector {
() => {
// Module: crate::opaque_types
// Provides: {"OpaqueTypeCollector"}
// Dependencies: {}
struct OpaqueTypeCollector < 'tcx > { tcx : TyCtxt < 'tcx > , opaques : Vec < LocalDefId > , # [doc = " The `DefId` of the item which we are collecting opaque types for."] item : LocalDefId , # [doc = " Avoid infinite recursion due to recursive declarations."] seen : FxHashSet < LocalDefId > , span : Option < Span > , mode : CollectionMode , }
};
}
