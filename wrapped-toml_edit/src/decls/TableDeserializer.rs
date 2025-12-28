macro_rules! deps {
    () => {
        KeyValuePairs!();
    };
}

macro_rules! TableDeserializer {
    () => {
        deps!();
        pub (crate) struct TableDeserializer { span : Option < std :: ops :: Range < usize > > , items : crate :: table :: KeyValuePairs , }
    };
}

TableDeserializer!();