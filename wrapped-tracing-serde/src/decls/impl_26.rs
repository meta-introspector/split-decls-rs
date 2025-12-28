macro_rules! deps {
    () => {
        AsSerde!();
        SerializeAttributes!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: span :: Attributes < 'a > { type Serializable = SerializeAttributes < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeAttributes (self) } }
    };
}

impl_26!()