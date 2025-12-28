macro_rules! deps {
    () => {
        Error!();
        SerializeValueArray!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeTuple for SerializeValueArray < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_329!()