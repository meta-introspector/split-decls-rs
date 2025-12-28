macro_rules! deps {
    () => {
        SerializeTupleStruct!();
        Serializer!();
        SerializerError!();
        ConstValue!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl ser :: SerializeTupleStruct for SerializeTupleStruct { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: List (self . 0)) } }
    };
}

impl_53!()