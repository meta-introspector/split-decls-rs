macro_rules! deps {
    () => {
        Error!();
        KeySerializer!();
        SerializeInlineTable!();
        Item!();
        Value!();
        SerializeMap!();
        MapValueSerializer!();
        InlineTable!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeMap for SerializeInlineTable { type Ok = crate :: InlineTable ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . key = Some (input . serialize (KeySerializer) ?) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let key = self . key . take () . unwrap () ; let mut is_none = false ; let value_serializer = MapValueSerializer :: new (& mut is_none) ; let res = value . serialize (value_serializer) ; match res { Ok (item) => { let item = crate :: Item :: Value (item) ; self . items . insert (key , item) ; } Err (e) => { if ! (e == Error :: unsupported_none () && is_none) { return Err (e) ; } } } ; Ok (()) } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeMap > :: Ok , < Self as serde_core :: ser :: SerializeMap > :: Error > { Ok (crate :: InlineTable :: with_pairs (self . items)) } }
    };
}

impl_368!();