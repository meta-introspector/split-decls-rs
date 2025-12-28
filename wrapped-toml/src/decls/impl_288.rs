macro_rules! deps {
    () => {
        SerializeDocumentTable!();
        SerializeMap!();
        Error!();
        KeySerializer!();
        Serializer!();
        SerializationStrategy!();
        ArrayOfTablesSerializer!();
        Table!();
        Buffer!();
        ValueSerializer!();
        Value!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeMap for SerializeDocumentTable < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut encoded_key = String :: new () ; input . serialize (KeySerializer { dst : & mut encoded_key , }) ? ; self . key = Some (encoded_key) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let encoded_key = self . key . take () . expect ("always called after `serialize_key`") ; match SerializationStrategy :: from (value) { SerializationStrategy :: Value => { let dst = self . table . body_mut () ; write ! (dst , "{encoded_key}") ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; let value_serializer = ValueSerializer :: with_style (dst , self . style) ; let dst = value . serialize (value_serializer) ? ; dst . newline () ? ; } SerializationStrategy :: ArrayOfTables => { self . table . has_children (true) ; let value_serializer = ArrayOfTablesSerializer :: new (self . buf , self . table . clone () , encoded_key , self . style ,) ; value . serialize (value_serializer) ? ; } SerializationStrategy :: Table | SerializationStrategy :: Unknown => { let child = self . buf . child_table (& mut self . table , encoded_key) ; let value_serializer = Serializer :: with_table (self . buf , child , self . style) ; value . serialize (value_serializer) ? ; } SerializationStrategy :: Skip => { } } Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
    };
}

impl_288!()