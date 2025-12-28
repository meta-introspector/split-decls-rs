macro_rules! deps {
    () => {
        Error!();
        ValueSerializeVec!();
        Table!();
        ValueSerializeVariant!();
        Value!();
        SerializeTupleVariant!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl ser :: SerializeTupleVariant for ValueSerializeVariant < ValueSerializeVec > { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (& mut self . inner , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let inner = ser :: SerializeSeq :: end (self . inner) ? ; let mut table = Table :: new () ; table . insert (self . variant . to_owned () , inner) ; Ok (Value :: Table (table)) } }
    };
}

impl_114!()