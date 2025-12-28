macro_rules! deps {
    () => {
        Error!();
        Result!();
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , Size > Deserialize < 'de > for EncodedPoint < Size > where Size : ModulusSize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let bytes = serdect :: slice :: deserialize_hex_or_bin_vec (deserializer) ? ; Self :: from_bytes (bytes) . map_err (de :: Error :: custom) } }
    };
}

impl_22!();