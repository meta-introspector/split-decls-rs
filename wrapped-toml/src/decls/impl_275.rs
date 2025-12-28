macro_rules! deps {
    () => {
        SerializeArrayOfTablesSerializer!();
        Buffer!();
        Error!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeTuple for SerializeArrayOfTablesSerializer < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
    };
}

impl_275!();