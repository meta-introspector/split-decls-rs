macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! VariantDeserializer {
    () => {
        deps!();
        struct VariantDeserializer < E > { value : Option < Value > , error : PhantomData < fn () -> E > , }
    };
}

VariantDeserializer!()