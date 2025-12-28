macro_rules! deps {
    () => {
        Path!();
        Wrap!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [doc = " Forwarding impl to preserve context."] impl < 'a , 'b , 'de , X , F > de :: EnumAccess < 'de > for Wrap < 'a , 'b , X , F > where X : de :: EnumAccess < 'de > + 'a , F : FnMut (Path) + 'b , { type Error = X :: Error ; type Variant = Wrap < 'a , 'b , X :: Variant , F > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , X :: Error > where V : DeserializeSeed < 'de > , { let callback = self . callback ; let path = self . path ; self . delegate . variant_seed (seed) . map (move | (v , vis) | (v , Wrap :: new (vis , callback , path))) } }
    };
}

impl_11!();