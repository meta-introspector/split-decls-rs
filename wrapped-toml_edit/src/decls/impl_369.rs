macro_rules! deps {
    () => {
        SerializeInlineTable!();
        MapValueSerializer!();
        Value!();
        Key!();
        Item!();
        Error!();
        InlineTable!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeStruct for SerializeInlineTable { type Ok = crate :: InlineTable ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeStruct > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut is_none = false ; let value_serializer = MapValueSerializer :: new (& mut is_none) ; let res = value . serialize (value_serializer) ; match res { Ok (item) => { let item = crate :: Item :: Value (item) ; self . items . insert (crate :: Key :: new (key) , item) ; } Err (e) => { if ! (e == Error :: unsupported_none () && is_none) { return Err (e) ; } } } ; Ok (()) } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeStruct > :: Ok , < Self as serde_core :: ser :: SerializeStruct > :: Error > { Ok (crate :: InlineTable :: with_pairs (self . items)) } }
    };
}

impl_369!();