macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! ValueDeserializer {
    () => {
        deps!();
        pub struct ValueDeserializer < E > { value : Value , error : PhantomData < fn () -> E > , }
    };
}

ValueDeserializer!();