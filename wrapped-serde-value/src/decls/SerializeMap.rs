macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        struct SerializeMap { map : BTreeMap < Value , Value > , key : Option < Value > , }
    };
}

SerializeMap!();