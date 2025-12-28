macro_rules! deps {
    () => {
        Value!();
        Error!();
        SerializationStrategy!();
        StructWalkValue!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeStruct for StructWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_field < T > (& mut self , _key : & 'static str , _value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (SerializationStrategy :: Value) } }
    };
}

impl_307!();