macro_rules! deps {
    () => {
        SerializeMap!();
        Table!();
        Error!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl ser :: SerializeStruct for SerializeMap { type Ok = Table ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeMap :: serialize_key (self , key) ? ; ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < Table , crate :: ser :: Error > { ser :: SerializeMap :: end (self) } }
    };
}

impl_384!()