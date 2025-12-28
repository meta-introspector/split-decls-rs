macro_rules! deps {
    () => {
        Value!();
        DeserializerError!();
        VariantDeserializer!();
        EnumDeserializer!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'de > EnumAccess < 'de > for EnumDeserializer { type Error = DeserializerError ; type Variant = VariantDeserializer ; # [inline] fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , VariantDeserializer) , DeserializerError > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
    };
}

impl_10!()