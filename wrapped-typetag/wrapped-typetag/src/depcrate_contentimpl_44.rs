// Generated macro for impl_44 (impl)
macro_rules! Depcrate_contentimpl_44 {
() => {
// Module: crate::content
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'de , E > EnumAccess < 'de > for EnumDeserializer < 'de , E > where E : de :: Error , { type Error = E ; type Variant = VariantDeserializer < 'de , Self :: Error > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , E > where V : DeserializeSeed < 'de > , { let visitor = VariantDeserializer { value : self . value , err : PhantomData , } ; seed . deserialize (ContentDeserializer :: new (self . variant)) . map (| v | (v , visitor)) } }
};
}
