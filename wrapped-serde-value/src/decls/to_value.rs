macro_rules! deps {
    () => {
        Value!();
        SerializerError!();
        Serializer!();
    };
}

macro_rules! to_value {
    () => {
        deps!();
        pub fn to_value < T : ser :: Serialize > (value : T) -> Result < Value , SerializerError > { value . serialize (Serializer) }
    };
}

to_value!()