// Generated macro for impl_337 (impl)
macro_rules! Depcrate_filters_corsimpl_337 {
() => {
// Module: crate::filters::cors
// Provides: {"impl_337"}
// Dependencies: {}
impl < F > WrapSealed < F > for Cors where F : Filter + Clone + Send + Sync + 'static , F :: Extract : Reply , F :: Error : CombineRejection < Rejection > , < F :: Error as CombineRejection < Rejection > > :: One : CombineRejection < Rejection > , { type Wrapped = CorsFilter < F > ; fn wrap (& self , inner : F) -> Self :: Wrapped { let config = self . config . clone () ; CorsFilter { config , inner } } }
};
}
