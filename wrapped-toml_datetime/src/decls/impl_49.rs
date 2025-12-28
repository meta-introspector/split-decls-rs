macro_rules! deps {
    () => {
        DatetimeOrTable!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: DeserializeSeed < 'de > for DatetimeOrTable < '_ , 'de > { type Value = () ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { deserializer . deserialize_any (self) } }
    };
}

impl_49!();