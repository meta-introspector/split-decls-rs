macro_rules! deps {
    () => {
        Value!();
        Error!();
        SerializeValueArray!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeTupleStruct for SerializeValueArray { type Ok = crate :: Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_341!()