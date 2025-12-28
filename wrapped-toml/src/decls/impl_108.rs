macro_rules! deps {
    () => {
        Value!();
        SerializeMap!();
        ValueSerializeMap!();
        Error!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl ser :: SerializeStruct for ValueSerializeMap { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeMap :: serialize_key (self , key) ? ; ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeMap :: end (self) } }
    };
}

impl_108!();