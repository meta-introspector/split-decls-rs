macro_rules! deps {
    () => {
        Serializer!();
        SerializeSeq!();
        ConstValue!();
        SerializerError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl ser :: SerializeSeq for SerializeSeq { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: List (self . 0)) } }
    };
}

impl_49!()