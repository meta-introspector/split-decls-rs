macro_rules! deps {
    () => {
        SlabVisitor!();
        Builder!();
        Slab!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'de , T > Visitor < 'de > for SlabVisitor < T > where T : Deserialize < 'de > , { type Value = Slab < T > ; fn expecting (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "a map") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut builder = Builder :: with_capacity (map . size_hint () . unwrap_or (0)) ; while let Some ((key , value)) = map . next_entry () ? { builder . pair (key , value) } Ok (builder . build ()) } }
    };
}

impl_5!();