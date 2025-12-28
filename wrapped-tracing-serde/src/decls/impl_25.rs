macro_rules! deps {
    () => {
        AsSerde!();
        SerializeEvent!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: Event < 'a > { type Serializable = SerializeEvent < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeEvent (self) } }
    };
}

impl_25!()