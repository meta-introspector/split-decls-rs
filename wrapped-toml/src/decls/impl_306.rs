macro_rules! deps {
    () => {
        Error!();
        SerializationStrategy!();
        SerializeMap!();
        StructWalkValue!();
        Value!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeMap for StructWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_key < T > (& mut self , _input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn serialize_value < T > (& mut self , _value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (SerializationStrategy :: Value) } }
    };
}

impl_306!()