macro_rules! deps {
    () => {
        UnitOnly!();
        KeyDeserializer!();
        Value!();
        Error!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: EnumAccess < 'de > for KeyDeserializer < 'de > { type Error = Error ; type Variant = UnitOnly < Self :: Error > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (unit_only) } }
    };
}

impl_129!()