macro_rules! deps {
    () => {
        ValueDeserializer!();
        Value!();
        EnumDeserializer!();
        VariantDeserializer!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'de , E > de :: EnumAccess < 'de > for EnumDeserializer < E > where E : de :: Error , { type Error = E ; type Variant = VariantDeserializer < Self :: Error > ; fn variant_seed < V > (self , seed : V ,) -> Result < (V :: Value , VariantDeserializer < Self :: Error >) , Self :: Error > where V : de :: DeserializeSeed < 'de > , { let visitor = VariantDeserializer { value : self . value , error : Default :: default () , } ; seed . deserialize (ValueDeserializer :: new (self . variant)) . map (| v | (v , visitor)) } }
    };
}

impl_19!();