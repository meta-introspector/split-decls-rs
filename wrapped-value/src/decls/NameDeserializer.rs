macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! NameDeserializer {
    () => {
        deps!();
        struct NameDeserializer { value : Name , }
    };
}

NameDeserializer!();