macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
        ArrayVecVisitor!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < 'de , A : Array > Deserialize < 'de > for ArrayVec < A > where A :: Item : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (ArrayVecVisitor (PhantomData)) } }
    };
}

impl_15!()