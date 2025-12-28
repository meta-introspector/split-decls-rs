macro_rules! deps {
    () => {
        AsSerde!();
        SerializeField!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for Field { type Serializable = SerializeField < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeField (self) } }
    };
}

impl_38!()