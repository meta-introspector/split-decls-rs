macro_rules! deps {
    () => {
        Deserializer!();
        MapDeserializer!();
        Array!();
        SeqDeserializer!();
        Value!();
        Error!();
        Table!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 'de > de :: Deserializer < 'de > for Value { type Error = crate :: de :: Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { match self { Self :: Boolean (v) => visitor . visit_bool (v) , Self :: Integer (n) => visitor . visit_i64 (n) , Self :: Float (n) => visitor . visit_f64 (n) , Self :: String (v) => visitor . visit_string (v) , Self :: Datetime (v) => visitor . visit_string (v . to_string ()) , Self :: Array (v) => { let len = v . len () ; let mut deserializer = SeqDeserializer :: new (v) ; let seq = visitor . visit_seq (& mut deserializer) ? ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (seq) } else { Err (de :: Error :: invalid_length (len , & "fewer elements in array")) } } Self :: Table (v) => { let len = v . len () ; let mut deserializer = MapDeserializer :: new (v) ; let map = visitor . visit_map (& mut deserializer) ? ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (map) } else { Err (de :: Error :: invalid_length (len , & "fewer elements in map")) } } } } # [inline] fn deserialize_enum < V > (self , _name : & 'static str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { match self { Self :: String (variant) => visitor . visit_enum (variant . into_deserializer ()) , Self :: Table (variant) => { if variant . is_empty () { Err (crate :: de :: Error :: custom ("wanted exactly 1 element, found 0 elements" , None ,)) } else if variant . len () != 1 { Err (crate :: de :: Error :: custom ("wanted exactly 1 element, more than 1 element" , None ,)) } else { let deserializer = MapDeserializer :: new (variant) ; visitor . visit_enum (deserializer) } } _ => Err (de :: Error :: invalid_type (de :: Unexpected :: UnitVariant , & "string only" ,)) , } } fn deserialize_option < V > (self , visitor : V) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { visitor . visit_some (self) } fn deserialize_newtype_struct < V > (self , _name : & 'static str , visitor : V ,) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { visitor . visit_newtype_struct (self) } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq bytes byte_buf map unit_struct tuple_struct struct tuple ignored_any identifier } }
    };
}

impl_87!();