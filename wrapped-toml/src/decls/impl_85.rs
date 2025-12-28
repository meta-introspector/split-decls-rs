macro_rules! deps {
    () => {
        Array!();
        Table!();
        Error!();
        Value!();
        Serializer!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl ser :: Serialize for Value { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match * self { Self :: String (ref s) => serializer . serialize_str (s) , Self :: Integer (i) => serializer . serialize_i64 (i) , Self :: Float (f) => serializer . serialize_f64 (f) , Self :: Boolean (b) => serializer . serialize_bool (b) , Self :: Datetime (ref s) => s . serialize (serializer) , Self :: Array (ref a) => a . serialize (serializer) , Self :: Table (ref t) => t . serialize (serializer) , } } }
    };
}

impl_85!();