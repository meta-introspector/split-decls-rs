// Generated macro for impl_339 (impl)
macro_rules! Depcrate_unstable_convertimpl_339 {
() => {
// Module: crate::unstable::convert
// Provides: {"impl_339"}
// Dependencies: {}
impl < 'tcx , T > Stable < 'tcx > for RangeInclusive < T > where T : Stable < 'tcx > , { type T = RangeInclusive < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { RangeInclusive :: new (self . start () . stable (tables , cx) , self . end () . stable (tables , cx)) } }
};
}
