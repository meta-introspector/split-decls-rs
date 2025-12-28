macro_rules! deps {
    () => {
        AsSerde!();
        SerializeId!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: span :: Id { type Serializable = SerializeId < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeId (self) } }
    };
}

impl_35!()