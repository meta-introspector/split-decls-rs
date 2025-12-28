macro_rules! deps {
    () => {
        SerializeValueArray!();
        Item!();
        Value!();
        Array!();
        ValueSerializer!();
        Error!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeSeq for SerializeValueArray { type Ok = crate :: Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let value = value . serialize (super :: ValueSerializer { }) ? ; self . values . push (crate :: Item :: Value (value)) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (crate :: Value :: Array (crate :: Array :: with_vec (self . values))) } }
    };
}

impl_339!();