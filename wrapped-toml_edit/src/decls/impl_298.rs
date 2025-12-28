macro_rules! deps {
    () => {
        Error!();
        UnitOnly!();
        KeyDeserializer!();
        Value!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: EnumAccess < 'de > for KeyDeserializer { type Error = Error ; type Variant = UnitOnly < < Self as serde_core :: de :: EnumAccess < 'de > > :: Error > ; fn variant_seed < T > (self , seed : T) -> Result < (< T as serde_core :: de :: DeserializeSeed < 'de > > :: Value , < Self as serde_core :: de :: EnumAccess < 'de > > :: Variant) , < Self as serde_core :: de :: EnumAccess < 'de > > :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (unit_only) } }
    };
}

impl_298!();