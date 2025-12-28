macro_rules! deps {
    () => {
        Value!();
        Name!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Value { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct ValueVisitor ; impl < 'de > Visitor < 'de > for ValueVisitor { type Value = Value ; # [inline] fn expecting (& self , formatter : & mut Formatter) -> fmt :: Result { formatter . write_str ("any valid value") } # [inline] fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Boolean (v)) } # [inline] fn visit_i64 < E > (self , v : i64) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Number (v . into ())) } # [inline] fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Number (v . into ())) } # [inline] fn visit_f64 < E > (self , v : f64) -> Result < Self :: Value , E > where E : DeError , { Ok (Number :: from_f64 (v) . map_or (Value :: Null , Value :: Number)) } # [inline] fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: String (v . to_string ())) } # [inline] fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: String (v)) } # [inline] fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Binary (v . to_vec () . into ())) } # [inline] fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Binary (v . into ())) } # [inline] fn visit_none < E > (self) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Null) } # [inline] fn visit_some < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : DeError , { Ok (Value :: Null) } fn visit_seq < A > (self , mut visitor : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut vec = Vec :: new () ; while let Some (elem) = visitor . next_element () ? { vec . push (elem) ; } Ok (Value :: List (vec)) } fn visit_map < A > (self , mut visitor : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut map = IndexMap :: new () ; while let Some ((name , value)) = visitor . next_entry () ? { match & value { Value :: String (value) if name == "$var" => { return Ok (Value :: Variable (Name :: new (value))) ; } _ => { map . insert (name , value) ; } } } Ok (Value :: Object (map)) } } deserializer . deserialize_any (ValueVisitor) } }
    };
}

impl_72!();