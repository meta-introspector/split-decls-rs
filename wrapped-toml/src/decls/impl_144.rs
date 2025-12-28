macro_rules! deps {
    () => {
        TableEnumDeserializer!();
        DeValue!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'i > TableEnumDeserializer < 'i > { pub (crate) fn new (value : DeValue < 'i > , span : core :: ops :: Range < usize >) -> Self { TableEnumDeserializer { value , span } } }
    };
}

impl_144!();