macro_rules! deps {
    () => {
        TableDeserializer!();
        DeTable!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'i > TableDeserializer < 'i > { pub (crate) fn new (items : DeTable < 'i > , span : core :: ops :: Range < usize >) -> Self { Self { span , items } } }
    };
}

impl_135!()