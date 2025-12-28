macro_rules! deps {
    () => {
        SerializeDatetime!();
        Error!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl serde_core :: ser :: SerializeStruct for SerializeDatetime { type Ok = crate :: Datetime ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeStruct > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . inner . serialize_field (key , value) . map_err (dt_err) ? ; Ok (()) } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeStruct > :: Ok , < Self as serde_core :: ser :: SerializeStruct > :: Error > { let value = self . inner . end () . map_err (dt_err) ? ; Ok (value) } }
    };
}

impl_364!()