// Generated macro for impl_202 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_202 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_202"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Primitive { type T = Primitive ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: Primitive :: Int (length , signed) => { Primitive :: Int { length : length . stable (tables , cx) , signed : * signed } } rustc_abi :: Primitive :: Float (length) => { Primitive :: Float { length : length . stable (tables , cx) } } rustc_abi :: Primitive :: Pointer (space) => Primitive :: Pointer (space . stable (tables , cx)) , } } }
};
}
