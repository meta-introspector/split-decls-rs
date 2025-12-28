macro_rules! deps {
    () => {
        ValueSerializeVec!();
        Value!();
        Array!();
        Error!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl ser :: SerializeSeq for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . vec . push (Value :: try_from (value) ?) ; Ok (()) } fn end (self) -> Result < Value , crate :: ser :: Error > { Ok (Value :: Array (self . vec)) } }
    };
}

impl_102!();