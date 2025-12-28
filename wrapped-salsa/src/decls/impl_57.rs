macro_rules! deps {
    () => {
        CycleHeads!();
        CycleHead!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for CycleHeads { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let vec : ThinVec < CycleHead > = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (CycleHeads (vec)) } }
    };
}

impl_57!()