macro_rules! deps {
    () => {
        Value!();
        Error!();
        Table!();
        SerializeMap!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeStruct for SerializeMap { type Ok = crate :: Value ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeStruct > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_field (key , value) , Self :: Table (s) => s . serialize_field (key , value) , } } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeMap > :: Ok , < Self as serde_core :: ser :: SerializeMap > :: Error > { match self { Self :: Datetime (s) => s . end () . map (| items | items . into ()) , Self :: Table (s) => s . end () . map (| items | items . into ()) , } } }
    };
}

impl_360!();