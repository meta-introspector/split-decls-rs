macro_rules! deps {
    () => {
        Serializer!();
        Value!();
        SerializeStructVariant!();
        SerializerError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl ser :: SerializeStructVariant for SerializeStructVariant { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > where T : ser :: Serialize , { let key = Value :: String (key . to_string ()) ; let value = value . serialize (Serializer) ? ; self . 1 . insert (key , value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = BTreeMap :: new () ; map . insert (self . 0 , Value :: Map (self . 1)) ; Ok (Value :: Map (map)) } }
    };
}

impl_44!();