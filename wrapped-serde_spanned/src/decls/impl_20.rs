macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : serde_core :: ser :: Serialize > serde_core :: ser :: Serialize for Spanned < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { self . value . serialize (serializer) } }
    };
}

impl_20!()