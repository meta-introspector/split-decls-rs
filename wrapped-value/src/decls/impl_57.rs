macro_rules! deps {
    () => {
        MapKeySerializer!();
        SerializerError!();
        Serializer!();
        SerializeMap!();
        ConstValue!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl ser :: SerializeMap for SerializeMap { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let key = key . serialize (MapKeySerializer) ? ; self . key = Some (key) ; Ok (()) } # [inline] fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . map . insert (self . key . take () . unwrap () , value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: Object (self . map)) } }
    };
}

impl_57!();