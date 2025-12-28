macro_rules! deps {
    () => {
        SerializeField!();
        AsSerde!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for Field { type Serializable = SerializeField < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeField (self) } }
    };
}

impl_30!()