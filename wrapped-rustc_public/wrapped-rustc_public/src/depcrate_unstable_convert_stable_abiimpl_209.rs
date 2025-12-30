// Generated macro for impl_209 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_209 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: ReprOptions { type T = ReprOptions ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { ReprOptions { int : self . int . map (| int | int . stable (tables , cx)) , align : self . align . map (| align | align . stable (tables , cx)) , pack : self . pack . map (| pack | pack . stable (tables , cx)) , flags : self . flags . stable (tables , cx) , } } }
};
}
