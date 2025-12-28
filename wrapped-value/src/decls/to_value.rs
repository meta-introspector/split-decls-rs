macro_rules! deps {
    () => {
        ConstValue!();
        SerializerError!();
        Serializer!();
    };
}

macro_rules! to_value {
    () => {
        deps!();
        # [doc = " Convert a `T` into `ConstValue` which is an enum that can represent any"] # [doc = " valid GraphQL data."] # [inline] pub fn to_value < T : ser :: Serialize > (value : T) -> Result < ConstValue , SerializerError > { value . serialize (Serializer) }
    };
}

to_value!()