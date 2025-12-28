macro_rules! deps {
    () => {
        AsSerde!();
        SerializeFieldSet!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a > AsSerde < 'a > for FieldSet { type Serializable = SerializeFieldSet < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeFieldSet (self) } }
    };
}

impl_39!();