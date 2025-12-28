macro_rules! deps {
    () => {
        AsSerde!();
        SerializeRecord!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: span :: Record < 'a > { type Serializable = SerializeRecord < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeRecord (self) } }
    };
}

impl_36!()