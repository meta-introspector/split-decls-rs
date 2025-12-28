macro_rules! deps {
    () => {
        SerializerError!();
        ConstValue!();
        SerializeStructVariant!();
        Name!();
        Serializer!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl ser :: SerializeStructVariant for SerializeStructVariant { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let key = Name :: new (key) ; let value = value . serialize (Serializer) ? ; self . 1 . insert (key , value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = IndexMap :: new () ; map . insert (self . 0 , ConstValue :: Object (self . 1)) ; Ok (ConstValue :: Object (map)) } }
    };
}

impl_61!()