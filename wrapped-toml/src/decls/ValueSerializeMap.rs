macro_rules! deps {
    () => {
        SerializeMap!();
    };
}

macro_rules! ValueSerializeMap {
    () => {
        deps!();
        pub (crate) struct ValueSerializeMap { ser : crate :: table :: SerializeMap , }
    };
}

ValueSerializeMap!();