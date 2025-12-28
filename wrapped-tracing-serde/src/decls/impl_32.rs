macro_rules! deps {
    () => {
        AsSerde!();
        SerializeMetadata!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: Metadata < 'a > { type Serializable = SerializeMetadata < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeMetadata (self) } }
    };
}

impl_32!()