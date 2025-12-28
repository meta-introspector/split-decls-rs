macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
    };
}

macro_rules! EnumDeserializer {
    () => {
        deps!();
        struct EnumDeserializer { variant : Name , value : Option < ConstValue > , }
    };
}

EnumDeserializer!()