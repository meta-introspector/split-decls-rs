// Generated macro for impl_14 (impl)
macro_rules! Depcrate_deimpl_14 {
() => {
// Module: crate::de
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'de , D > de :: EnumAccess < 'de > for EnumAccess < D > where D : de :: EnumAccess < 'de > , { type Error = D :: Error ; type Variant = VariantAccess < D :: Variant > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , D :: Error > where V : de :: DeserializeSeed < 'de > , { let param = self . param ; self . delegate . variant_seed (DeserializeSeed :: new (seed , param)) . map (| (v , vis) | (v , VariantAccess :: new (vis , param))) } }
};
}
