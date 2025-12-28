macro_rules! deps {
    () => {
        Name!();
        EnumDeserializer!();
        DeserializerError!();
        Value!();
        ConstValue!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'de > de :: Deserializer < 'de > for ConstValue { type Error = DeserializerError ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < < V as Visitor < 'de > > :: Value , Self :: Error > where V : Visitor < 'de > , { match self { ConstValue :: Null => visitor . visit_unit () , ConstValue :: Number (v) => v . deserialize_any (visitor) . map_err (| err | DeserializerError (err . to_string ())) , ConstValue :: String (v) => visitor . visit_str (& v) , ConstValue :: Boolean (v) => visitor . visit_bool (v) , ConstValue :: Binary (bytes) => visitor . visit_bytes (& bytes) , ConstValue :: Enum (v) => visitor . visit_str (v . as_str ()) , ConstValue :: List (v) => visit_array (v , visitor) , ConstValue :: Object (v) => visit_object (v , visitor) , } } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct seq tuple tuple_struct map struct identifier ignored_any } # [inline] fn deserialize_option < V > (self , visitor : V) -> Result < < V as Visitor < 'de > > :: Value , Self :: Error > where V : Visitor < 'de > , { match self { ConstValue :: Null => visitor . visit_none () , _ => visitor . visit_some (self) , } } # [inline] fn deserialize_newtype_struct < V > (self , _name : & 'static str , visitor : V ,) -> Result < < V as Visitor < 'de > > :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_newtype_struct (self) } fn deserialize_enum < V > (self , _name : & 'static str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < < V as Visitor < 'de > > :: Value , Self :: Error > where V : Visitor < 'de > , { let (variant , value) = match self { ConstValue :: Object (value) => { let mut iter = value . into_iter () ; let (variant , value) = match iter . next () { Some (v) => v , None => { return Err (serde :: de :: Error :: invalid_value (Unexpected :: Map , & "map with a single key" ,)) ; } } ; if iter . next () . is_some () { return Err (serde :: de :: Error :: invalid_value (Unexpected :: Map , & "map with a single key" ,)) ; } (variant , Some (value)) } ConstValue :: String (variant) => (Name :: new (variant) , None) , ConstValue :: Enum (variant) => (variant , None) , other => { return Err (DeserializerError :: invalid_type (other . unexpected () , & "string or map" ,)) ; } } ; visitor . visit_enum (EnumDeserializer { variant , value }) } # [inline] fn is_human_readable (& self) -> bool { true } }
    };
}

impl_8!();