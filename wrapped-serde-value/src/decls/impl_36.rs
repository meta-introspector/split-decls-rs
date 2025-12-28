macro_rules! deps {
    () => {
        SerializeTupleStruct!();
        Value!();
        SerializerError!();
        Serializer!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl ser :: SerializeTupleStruct for SerializeTupleStruct { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Seq (self . 0)) } }
    };
}

impl_36!();