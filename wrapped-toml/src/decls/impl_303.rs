macro_rules! deps {
    () => {
        ArrayWalkValue!();
        Error!();
        SerializationStrategy!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeTuple for ArrayWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . serialize_element (value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
    };
}

impl_303!()