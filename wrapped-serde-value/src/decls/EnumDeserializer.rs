macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! EnumDeserializer {
    () => {
        deps!();
        struct EnumDeserializer < E > { variant : Value , value : Option < Value > , error : PhantomData < fn () -> E > , }
    };
}

EnumDeserializer!()