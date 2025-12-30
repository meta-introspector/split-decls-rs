// Generated macro for impl_27 (impl)
macro_rules! Depcrate_tupleimpl_27 {
() => {
// Module: crate::tuple
// Provides: {"impl_27"}
// Dependencies: {}
impl < S , L1 > Layer < S > for (L1 ,) where L1 : Layer < S > , { type Service = L1 :: Service ; fn layer (& self , service : S) -> Self :: Service { let (l1 ,) = self ; l1 . layer (service) } }
};
}
