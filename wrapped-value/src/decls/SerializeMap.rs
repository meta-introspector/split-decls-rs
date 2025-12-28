macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        struct SerializeMap { map : IndexMap < Name , ConstValue > , key : Option < Name > , }
    };
}

SerializeMap!()