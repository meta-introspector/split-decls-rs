// Generated macro for impl_30 (impl)
macro_rules! Depcrate_tupleimpl_30 {
() => {
// Module: crate::tuple
// Provides: {"impl_30"}
// Dependencies: {}
impl < S , L1 , L2 , L3 , L4 > Layer < S > for (L1 , L2 , L3 , L4) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < L4 :: Service > , L4 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3 , l4) = self ; l1 . layer ((l2 , l3 , l4) . layer (service)) } }
};
}
