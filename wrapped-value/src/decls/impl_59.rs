macro_rules! deps {
    () => {
        Serializer!();
        ConstValue!();
        Name!();
        SerializeStruct!();
        SerializerError!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl ser :: SerializeStruct for SerializeStruct { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let key = Name :: new (key) ; let value = value . serialize (Serializer) ? ; self . 0 . insert (key , value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: Object (self . 0)) } }
    };
}

impl_59!()