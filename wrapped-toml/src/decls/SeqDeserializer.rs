macro_rules! deps {
    () => {
        IntoIter!();
        Value!();
    };
}

macro_rules! SeqDeserializer {
    () => {
        deps!();
        pub (crate) struct SeqDeserializer { iter : vec :: IntoIter < Value > , }
    };
}

SeqDeserializer!()