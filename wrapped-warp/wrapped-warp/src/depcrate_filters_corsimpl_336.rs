// Generated macro for impl_336 (impl)
macro_rules! Depcrate_filters_corsimpl_336 {
() => {
// Module: crate::filters::cors
// Provides: {"impl_336"}
// Dependencies: {}
impl < F > WrapSealed < F > for Builder where F : Filter + Clone + Send + Sync + 'static , F :: Extract : Reply , F :: Error : CombineRejection < Rejection > , < F :: Error as CombineRejection < Rejection > > :: One : CombineRejection < Rejection > , { type Wrapped = CorsFilter < F > ; fn wrap (& self , inner : F) -> Self :: Wrapped { let Cors { config } = self . clone () . build () ; CorsFilter { config , inner } } }
};
}
