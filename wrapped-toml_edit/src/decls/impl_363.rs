macro_rules! deps {
    () => {
        SerializeDatetime!();
        Error!();
        SerializeMap!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeMap for SerializeDatetime { type Ok = crate :: Datetime ; type Error = Error ; fn serialize_key < T > (& mut self , _input : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { unreachable ! ("datetimes should only be serialized as structs, not maps") } fn serialize_value < T > (& mut self , _value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { unreachable ! ("datetimes should only be serialized as structs, not maps") } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeMap > :: Ok , < Self as serde_core :: ser :: SerializeMap > :: Error > { unreachable ! ("datetimes should only be serialized as structs, not maps") } }
    };
}

impl_363!();