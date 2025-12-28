macro_rules! deps {
    () => {
        Result!();
        True!();
        Error!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for True { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bool (True) } }
    };
}

impl_106!();