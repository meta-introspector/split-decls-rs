macro_rules! deps {
    () => {
        SerializerError!();
        Serializer!();
        ConstValue!();
        SerializeTupleVariant!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 1 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = IndexMap :: new () ; map . insert (self . 0 , ConstValue :: List (self . 1)) ; Ok (ConstValue :: Object (map)) } }
    };
}

impl_55!();