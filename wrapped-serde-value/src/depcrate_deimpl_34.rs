// Generated macro for impl_34 (impl)
macro_rules! Depcrate_deimpl_34 {
() => {
// Module: crate::de
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'de , E > de :: EnumAccess < 'de > for EnumDeserializer < E > where E : de :: Error , { type Error = E ; type Variant = VariantDeserializer < Self :: Error > ; fn variant_seed < V > (self , seed : V ,) -> Result < (V :: Value , VariantDeserializer < Self :: Error >) , Self :: Error > where V : de :: DeserializeSeed < 'de > , { let visitor = VariantDeserializer { value : self . value , error : Default :: default () , } ; seed . deserialize (ValueDeserializer :: new (self . variant)) . map (| v | (v , visitor)) } }
};
}
