// Generated macro for impl_29 (impl)
macro_rules! Depcrate_tupleimpl_29 {
() => {
// Module: crate::tuple
// Provides: {"impl_29"}
// Dependencies: {}
impl < S , L1 , L2 , L3 > Layer < S > for (L1 , L2 , L3) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3) = self ; l1 . layer ((l2 , l3) . layer (service)) } }
};
}
