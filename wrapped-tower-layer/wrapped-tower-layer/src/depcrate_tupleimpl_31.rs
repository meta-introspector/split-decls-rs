// Generated macro for impl_31 (impl)
macro_rules! Depcrate_tupleimpl_31 {
() => {
// Module: crate::tuple
// Provides: {"impl_31"}
// Dependencies: {}
impl < S , L1 , L2 , L3 , L4 , L5 > Layer < S > for (L1 , L2 , L3 , L4 , L5) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < L4 :: Service > , L4 : Layer < L5 :: Service > , L5 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3 , l4 , l5) = self ; l1 . layer ((l2 , l3 , l4 , l5) . layer (service)) } }
};
}
