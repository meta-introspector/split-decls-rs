// Generated macro for impl_42 (impl)
macro_rules! Depcrate_tupleimpl_42 {
() => {
// Module: crate::tuple
// Provides: {"impl_42"}
// Dependencies: {}
# [rustfmt :: skip] impl < S , L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9 , L10 , L11 , L12 , L13 , L14 , L15 , L16 > Layer < S > for (L1 , L2 , L3 , L4 , L5 , L6 , L7 , L8 , L9 , L10 , L11 , L12 , L13 , L14 , L15 , L16) where L1 : Layer < L2 :: Service > , L2 : Layer < L3 :: Service > , L3 : Layer < L4 :: Service > , L4 : Layer < L5 :: Service > , L5 : Layer < L6 :: Service > , L6 : Layer < L7 :: Service > , L7 : Layer < L8 :: Service > , L8 : Layer < L9 :: Service > , L9 : Layer < L10 :: Service > , L10 : Layer < L11 :: Service > , L11 : Layer < L12 :: Service > , L12 : Layer < L13 :: Service > , L13 : Layer < L14 :: Service > , L14 : Layer < L15 :: Service > , L15 : Layer < L16 :: Service > , L16 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 , l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9 , l10 , l11 , l12 , l13 , l14 , l15 , l16) = self ; l1 . layer ((l2 , l3 , l4 , l5 , l6 , l7 , l8 , l9 , l10 , l11 , l12 , l13 , l14 , l15 , l16) . layer (service)) } }
};
}
