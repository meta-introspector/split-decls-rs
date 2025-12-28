macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! MapDeserializer {
    () => {
        deps!();
        struct MapDeserializer { iter : < IndexMap < Name , ConstValue > as IntoIterator > :: IntoIter , value : Option < ConstValue > , }
    };
}

MapDeserializer!()