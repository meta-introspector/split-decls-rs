macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! deserialize_from {
    () => {
        deps!();
        # [doc = " Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute"] fn deserialize_from (type_from : & syn :: Type) -> Fragment { quote_block ! { _serde ::# private :: Result :: map (<# type_from as _serde :: Deserialize >:: deserialize (__deserializer) , _serde ::# private :: From :: from) } }
    };
}

deserialize_from!();