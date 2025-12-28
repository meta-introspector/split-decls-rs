macro_rules! deps {
    () => {
        Deserializer!();
        TrackedSeed!();
        Path!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a , 'de , X , F > DeserializeSeed < 'de > for TrackedSeed < 'a , X , F > where X : DeserializeSeed < 'de > , F : FnMut (Path) , { type Value = X :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < X :: Value , D :: Error > where D : de :: Deserializer < 'de > , { self . seed . deserialize (Deserializer { de : deserializer , callback : self . callback , path : self . path , }) } }
    };
}

impl_21!()