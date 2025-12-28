macro_rules! deps {
    () => {
        Error!();
        SerializeMap!();
        Table!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeMap for SerializeMap < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_key (input) , Self :: Table (s) => s . serialize_key (input) , } } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_value (value) , Self :: Table (s) => s . serialize_value (value) , } } fn end (self) -> Result < Self :: Ok , Self :: Error > { match self { Self :: Datetime (s) => s . end () , Self :: Table (s) => s . end () , } } }
    };
}

impl_340!();