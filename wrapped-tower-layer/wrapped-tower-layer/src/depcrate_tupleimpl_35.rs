// Generated macro for impl_35 (impl)
macro_rules! Depcrate_tupleimpl_35 {
() => {
// Module: crate::tuple
// Provides: {"impl_35"}
// Dependencies: {}
impl < S , L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9 > Layer < S > for (L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < L4 :: Service > , L4 : Layer < L5 :: Service > , L5 : Layer < L6 :: Service > , L6 : Layer < L7 :: Service > , L7 : Layer < L8 :: Service > , L8 : Layer < L9 :: Service > , L9 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9) = self ; l1 . layer ((l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9) . layer (service)) } }
};
}
