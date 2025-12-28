macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! ValueSerializeVec {
    () => {
        deps!();
        pub (crate) struct ValueSerializeVec { vec : Vec < Value > , }
    };
}

ValueSerializeVec!()