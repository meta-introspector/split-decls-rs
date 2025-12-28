macro_rules! deps {
    () => {
        SerializerError!();
        Value!();
        SerializeTupleVariant!();
        Serializer!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 1 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = BTreeMap :: new () ; map . insert (self . 0 , Value :: Seq (self . 1)) ; Ok (Value :: Map (map)) } }
    };
}

impl_38!()