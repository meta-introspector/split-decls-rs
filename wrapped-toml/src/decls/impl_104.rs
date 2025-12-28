macro_rules! deps {
    () => {
        Error!();
        ValueSerializeVec!();
        Value!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl ser :: SerializeTupleStruct for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
    };
}

impl_104!();