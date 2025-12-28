macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
    };
}

macro_rules! MapDeserializer {
    () => {
        deps!();
        struct MapDeserializer { iter : < IndexMap < Name , ConstValue > as IntoIterator > :: IntoIter , value : Option < ConstValue > , }
    };
}

MapDeserializer!();