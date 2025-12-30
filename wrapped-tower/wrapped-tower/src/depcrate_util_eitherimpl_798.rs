// Generated macro for impl_798 (impl)
macro_rules! Depcrate_util_eitherimpl_798 {
() => {
// Module: crate::util::either
// Provides: {"impl_798"}
// Dependencies: {}
impl < S , A , B > Layer < S > for Either < A , B > where A : Layer < S > , B : Layer < S > , { type Service = Either < A :: Service , B :: Service > ; fn layer (& self , inner : S) -> Self :: Service { match self { Either :: Left (layer) => Either :: Left (layer . layer (inner)) , Either :: Right (layer) => Either :: Right (layer . layer (inner)) , } } }
};
}
