macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! MapKeyDeserializer {
    () => {
        deps!();
        struct MapKeyDeserializer { key : Name , }
    };
}

MapKeyDeserializer!()