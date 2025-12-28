macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for DatabaseKeyIndex { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let (key_index , ingredient_index) = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (DatabaseKeyIndex { key_index , ingredient_index , }) } }
    };
}

impl_225!()