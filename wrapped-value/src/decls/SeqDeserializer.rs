macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! SeqDeserializer {
    () => {
        deps!();
        struct SeqDeserializer { iter : vec :: IntoIter < ConstValue > , }
    };
}

SeqDeserializer!();