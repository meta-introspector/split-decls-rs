macro_rules! deps {
    () => {
        Table!();
        Error!();
        ValueSerializeMap!();
        Value!();
        SerializeMap!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl ser :: SerializeMap for ValueSerializeMap { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . ser . serialize_key (key) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . ser . serialize_value (value) } fn end (self) -> Result < Value , crate :: ser :: Error > { self . ser . end () . map (Value :: Table) } }
    };
}

impl_107!()