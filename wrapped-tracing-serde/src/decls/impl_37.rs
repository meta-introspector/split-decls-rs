macro_rules! deps {
    () => {
        AsSerde!();
        SerializeLevel!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for Level { type Serializable = SerializeLevel < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeLevel (self) } }
    };
}

impl_37!()