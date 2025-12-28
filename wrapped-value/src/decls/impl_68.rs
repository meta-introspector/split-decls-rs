macro_rules! deps {
    () => {
        ConstValue!();
        Value!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for ConstValue { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct ValueVisitor ; impl < 'de > Visitor < 'de > for ValueVisitor { type Value = ConstValue ; # [inline] fn expecting (& self , formatter : & mut Formatter) -> fmt :: Result { formatter . write_str ("any valid value") } # [inline] fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Boolean (v)) } # [inline] fn visit_i64 < E > (self , v : i64) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Number (v . into ())) } # [inline] fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Number (v . into ())) } # [inline] fn visit_f64 < E > (self , v : f64) -> Result < Self :: Value , E > where E : DeError , { Ok (Number :: from_f64 (v) . map_or (ConstValue :: Null , ConstValue :: Number)) } # [inline] fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: String (v . to_string ())) } # [inline] fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: String (v)) } # [inline] fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Binary (v . to_vec () . into ())) } # [inline] fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Binary (v . into ())) } # [inline] fn visit_none < E > (self) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Null) } # [inline] fn visit_some < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : DeError , { Ok (ConstValue :: Null) } fn visit_seq < A > (self , mut visitor : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut vec = Vec :: new () ; while let Some (elem) = visitor . next_element () ? { vec . push (elem) ; } Ok (ConstValue :: List (vec)) } fn visit_map < A > (self , mut visitor : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut map = IndexMap :: new () ; while let Some ((name , value)) = visitor . next_entry () ? { map . insert (name , value) ; } Ok (ConstValue :: Object (map)) } } deserializer . deserialize_any (ValueVisitor) } }
    };
}

impl_68!();