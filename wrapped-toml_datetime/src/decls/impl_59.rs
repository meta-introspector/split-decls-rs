macro_rules! deps {
    () => {
        DatetimeSerializer!();
        Datetime!();
        SerializerError!();
        DatetimeFieldSerializer!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl DatetimeSerializer { # [doc = " Create a serializer to emit [`Datetime`][crate::Datetime]"] pub fn new () -> Self { Self { value : None } } # [doc = " See [`serde_core::ser::SerializeStruct::serialize_field`]"] pub fn serialize_field < T > (& mut self , key : & 'static str , value : & T ,) -> Result < () , SerializerError > where T : serde_core :: ser :: Serialize + ? Sized , { if key == crate :: datetime :: FIELD { self . value = Some (value . serialize (DatetimeFieldSerializer :: default ()) ?) ; } Ok (()) } # [doc = " See [`serde_core::ser::SerializeStruct::end`]"] pub fn end (self) -> Result < crate :: Datetime , SerializerError > { self . value . ok_or (SerializerError :: InvalidProtocol) } }
    };
}

impl_59!()