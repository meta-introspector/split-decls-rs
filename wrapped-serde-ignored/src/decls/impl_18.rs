macro_rules! deps {
    () => {
        CaptureKey!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , 'de , X > de :: EnumAccess < 'de > for CaptureKey < 'a , X > where X : de :: EnumAccess < 'de > , { type Error = X :: Error ; type Variant = X :: Variant ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , X :: Error > where V : DeserializeSeed < 'de > , { self . delegate . variant_seed (CaptureKey :: new (seed , self . key)) } }
    };
}

impl_18!();