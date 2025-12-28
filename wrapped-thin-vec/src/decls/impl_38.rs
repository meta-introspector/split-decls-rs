macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : serde :: Serialize > serde :: Serialize for ThinVec < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . collect_seq (self . as_slice ()) } }
    };
}

impl_38!();