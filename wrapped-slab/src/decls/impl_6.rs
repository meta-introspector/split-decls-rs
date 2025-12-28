macro_rules! deps {
    () => {
        Slab!();
        SlabVisitor!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'de , T > Deserialize < 'de > for Slab < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (SlabVisitor (PhantomData)) } }
    };
}

impl_6!();