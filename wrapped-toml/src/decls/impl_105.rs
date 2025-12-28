macro_rules! deps {
    () => {
        Value!();
        ValueSerializeVec!();
        Error!();
        SerializeTupleVariant!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl ser :: SerializeTupleVariant for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
    };
}

impl_105!()