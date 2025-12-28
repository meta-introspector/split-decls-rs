macro_rules! deps {
    () => {
        Deserializer!();
        Table!();
        Entry!();
        Array!();
        Error!();
        Value!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for Value { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct ValueVisitor ; impl < 'de > de :: Visitor < 'de > for ValueVisitor { type Value = Value ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("any valid TOML value") } fn visit_bool < E > (self , value : bool) -> Result < Value , E > { Ok (Value :: Boolean (value)) } fn visit_i64 < E > (self , value : i64) -> Result < Value , E > { Ok (Value :: Integer (value)) } fn visit_u64 < E : de :: Error > (self , value : u64) -> Result < Value , E > { if i64 :: try_from (value) . is_ok () { Ok (Value :: Integer (value as i64)) } else { Err (de :: Error :: custom ("u64 value was too large")) } } fn visit_u32 < E > (self , value : u32) -> Result < Value , E > { Ok (Value :: Integer (value . into ())) } fn visit_i32 < E > (self , value : i32) -> Result < Value , E > { Ok (Value :: Integer (value . into ())) } fn visit_f64 < E > (self , value : f64) -> Result < Value , E > { Ok (Value :: Float (value)) } fn visit_str < E > (self , value : & str) -> Result < Value , E > { Ok (Value :: String (value . into ())) } fn visit_string < E > (self , value : String) -> Result < Value , E > { Ok (Value :: String (value)) } fn visit_some < D > (self , deserializer : D) -> Result < Value , D :: Error > where D : de :: Deserializer < 'de > , { de :: Deserialize :: deserialize (deserializer) } fn visit_seq < V > (self , mut visitor : V) -> Result < Value , V :: Error > where V : de :: SeqAccess < 'de > , { let mut vec = Vec :: new () ; while let Some (elem) = visitor . next_element () ? { vec . push (elem) ; } Ok (Value :: Array (vec)) } fn visit_map < V > (self , mut visitor : V) -> Result < Value , V :: Error > where V : de :: MapAccess < 'de > , { let key = match toml_datetime :: de :: VisitMap :: next_key_seed (& mut visitor) ? { Some (toml_datetime :: de :: VisitMap :: Datetime (datetime)) => { return Ok (Value :: Datetime (datetime)) ; } None => return Ok (Value :: Table (Table :: new ())) , Some (toml_datetime :: de :: VisitMap :: Key (key)) => key , } ; let mut map = Table :: new () ; map . insert (key . into_owned () , visitor . next_value () ?) ; while let Some (key) = visitor . next_key :: < String > () ? { if let crate :: map :: Entry :: Vacant (vacant) = map . entry (& key) { vacant . insert (visitor . next_value () ?) ; } else { let msg = format ! ("duplicate key: `{key}`") ; return Err (de :: Error :: custom (msg)) ; } } Ok (Value :: Table (map)) } } deserializer . deserialize_any (ValueVisitor) } }
    };
}

impl_86!()