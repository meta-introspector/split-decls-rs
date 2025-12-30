// Generated macro for impl_36 (impl)
macro_rules! Depcrate_tupleimpl_36 {
() => {
// Module: crate::tuple
// Provides: {"impl_36"}
// Dependencies: {}
impl < S , L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9 , L10 > Layer < S > for (L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9 , L10) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < L4 :: Service > , L4 : Layer < L5 :: Service > , L5 : Layer < L6 :: Service > , L6 : Layer < L7 :: Service > , L7 : Layer < L8 :: Service > , L8 : Layer < L9 :: Service > , L9 : Layer < L10 :: Service > , L10 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9 , l10) = self ; l1 . layer ((l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9 , l10) . layer (service)) } }
};
}
