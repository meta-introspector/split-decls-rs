macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        pub (crate) struct SerializeMap { map : Table , next_key : Option < String > , }
    };
}

SerializeMap!()