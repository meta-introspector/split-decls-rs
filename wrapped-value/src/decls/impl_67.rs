macro_rules! deps {
    () => {
        ConstValue!();
        Serializer!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Serialize for ConstValue { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { ConstValue :: Null => serializer . serialize_none () , ConstValue :: Number (v) => v . serialize (serializer) , ConstValue :: String (v) => serializer . serialize_str (v) , ConstValue :: Boolean (v) => serializer . serialize_bool (* v) , ConstValue :: Binary (v) => serializer . serialize_bytes (v) , ConstValue :: Enum (v) => serializer . serialize_str (v) , ConstValue :: List (v) => v . serialize (serializer) , ConstValue :: Object (v) => { # [cfg (feature = "raw_value")] if v . len () == 1 { if let Some (ConstValue :: String (v)) = v . get (RAW_VALUE_TOKEN) { if let Ok (v) = serde_json :: value :: RawValue :: from_string (v . clone ()) { return v . serialize (serializer) ; } } } v . serialize (serializer) } } } }
    };
}

impl_67!();