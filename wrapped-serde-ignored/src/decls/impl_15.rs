macro_rules! deps {
    () => {
        Deserializer!();
        CaptureKey!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Forwarding impl."] impl < 'a , 'de , X > DeserializeSeed < 'de > for CaptureKey < 'a , X > where X : DeserializeSeed < 'de > , { type Value = X :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < X :: Value , D :: Error > where D : de :: Deserializer < 'de > , { self . delegate . deserialize (CaptureKey :: new (deserializer , self . key)) } }
    };
}

impl_15!();