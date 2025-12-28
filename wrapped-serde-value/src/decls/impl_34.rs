macro_rules! deps {
    () => {
        SerializeTuple!();
        SerializerError!();
        Value!();
        Serializer!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ser :: SerializeTuple for SerializeTuple { type Ok = Value ; type Error = SerializerError ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Seq (self . 0)) } }
    };
}

impl_34!()