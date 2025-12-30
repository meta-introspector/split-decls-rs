// Generated macro for impl_28 (impl)
macro_rules! Depcrate_tupleimpl_28 {
() => {
// Module: crate::tuple
// Provides: {"impl_28"}
// Dependencies: {}
impl < S , L1 , L2 > Layer < S > for (L1 , L2) where L1 : Layer < L2 :: Service > , L2 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2) = self ; l1 . layer (l2 . layer (service)) } }
};
}
