macro_rules! deps {
    () => {
        Value!();
        Error!();
        Table!();
        SerializeMap!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeMap for SerializeMap { type Ok = crate :: Value ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_key (input) , Self :: Table (s) => s . serialize_key (input) , } } fn serialize_value < T > (& mut self , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeMap > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_value (value) , Self :: Table (s) => s . serialize_value (value) , } } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeMap > :: Ok , < Self as serde_core :: ser :: SerializeMap > :: Error > { match self { Self :: Datetime (s) => s . end () . map (| items | items . into ()) , Self :: Table (s) => s . end () . map (| items | items . into ()) , } } }
    };
}

impl_359!()