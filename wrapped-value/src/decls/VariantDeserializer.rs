macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! VariantDeserializer {
    () => {
        deps!();
        struct VariantDeserializer { value : Option < ConstValue > , }
    };
}

VariantDeserializer!();