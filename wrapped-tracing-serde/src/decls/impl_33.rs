macro_rules! deps {
    () => {
        SerializeEvent!();
        AsSerde!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: Event < 'a > { type Serializable = SerializeEvent < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeEvent (self) } }
    };
}

impl_33!()