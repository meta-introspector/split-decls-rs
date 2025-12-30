// Generated macro for impl_336 (impl)
macro_rules! Depcrate_unstable_convertimpl_336 {
() => {
// Module: crate::unstable::convert
// Provides: {"impl_336"}
// Dependencies: {}
impl < 'tcx , T , E > Stable < 'tcx > for Result < T , E > where T : Stable < 'tcx > , E : Stable < 'tcx > , { type T = Result < T :: T , E :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { Ok (val) => Ok (val . stable (tables , cx)) , Err (error) => Err (error . stable (tables , cx)) , } } }
};
}
