macro_rules! deps {
    () => {
        SerializeTupleVariant!();
        Error!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeTupleVariant for SerializeTupleVariant < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (& mut self . inner , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let dst = self . inner . end () ? ; dst . space () ? ; dst . close_inline_table () ? ; Ok (dst) } }
    };
}

impl_333!()