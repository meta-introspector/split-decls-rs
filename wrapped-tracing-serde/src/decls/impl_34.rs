macro_rules! deps {
    () => {
        SerializeAttributes!();
        AsSerde!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for tracing_core :: span :: Attributes < 'a > { type Serializable = SerializeAttributes < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeAttributes (self) } }
    };
}

impl_34!();