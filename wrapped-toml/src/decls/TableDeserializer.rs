macro_rules! deps {
    () => {
        DeTable!();
    };
}

macro_rules! TableDeserializer {
    () => {
        deps!();
        pub (crate) struct TableDeserializer < 'i > { span : core :: ops :: Range < usize > , items : DeTable < 'i > , }
    };
}

TableDeserializer!();