macro_rules! deps {
    () => {
        Error!();
        Table!();
        SerializeStructVariant!();
        Value!();
        ValueSerializeVariant!();
        ValueSerializeMap!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl ser :: SerializeStructVariant for ValueSerializeVariant < ValueSerializeMap > { type Ok = Value ; type Error = crate :: ser :: Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeStruct :: serialize_field (& mut self . inner , key , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let inner = ser :: SerializeStruct :: end (self . inner) ? ; let mut table = Table :: new () ; table . insert (self . variant . to_owned () , inner) ; Ok (Value :: Table (table)) } }
    };
}

impl_115!();