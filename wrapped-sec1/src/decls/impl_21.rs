macro_rules! deps {
    () => {
        Error!();
        Result!();
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < Size > Serialize for EncodedPoint < Size > where Size : ModulusSize , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serdect :: slice :: serialize_hex_upper_or_bin (& self . as_bytes () , serializer) } }
    };
}

impl_21!();