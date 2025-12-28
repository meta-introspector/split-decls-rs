macro_rules! deps {
    () => {
        Value!();
        SerializeValueArray!();
        Error!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeTuple for SerializeValueArray { type Ok = crate :: Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_340!();