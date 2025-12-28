macro_rules! deps {
    () => {
        Value!();
        SerializerError!();
        Serializer!();
        SerializeMap!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl ser :: SerializeMap for SerializeMap { type Ok = Value ; type Error = SerializerError ; fn serialize_key < T : ? Sized > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let key = key . serialize (Serializer) ? ; self . key = Some (key) ; Ok (()) } fn serialize_value < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . map . insert (self . key . take () . unwrap () , value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Map (self . map)) } }
    };
}

impl_40!()