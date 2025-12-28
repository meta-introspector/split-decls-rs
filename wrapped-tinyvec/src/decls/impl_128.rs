macro_rules! deps {
    () => {
        TinyVec!();
        TinyVecVisitor!();
        Array!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < 'de , A : Array > Deserialize < 'de > for TinyVec < A > where A :: Item : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (TinyVecVisitor (PhantomData)) } }
    };
}

impl_128!()