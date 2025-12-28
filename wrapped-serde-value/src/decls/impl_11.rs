macro_rules! deps {
    () => {
        Value!();
        ValueVisitor!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for Value { fn deserialize < D : de :: Deserializer < 'de > > (d : D) -> Result < Self , D :: Error > { d . deserialize_any (ValueVisitor) } }
    };
}

impl_11!()