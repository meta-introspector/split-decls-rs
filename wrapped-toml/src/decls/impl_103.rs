macro_rules! deps {
    () => {
        ValueSerializeVec!();
        Value!();
        Error!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl ser :: SerializeTuple for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
    };
}

impl_103!()