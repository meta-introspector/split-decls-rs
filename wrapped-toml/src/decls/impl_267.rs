macro_rules! deps {
    () => {
        SerializeTupleVariant!();
        SerializeDocumentTupleVariant!();
        Buffer!();
        Error!();
        ValueSerializer!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeTupleVariant for SerializeDocumentTupleVariant < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let dst = self . table . body_mut () ; if self . style . multiline_array { dst . newline () ? ; write ! (dst , "    ") ? ; } else { if self . seen_value { dst . val_sep () ? ; dst . space () ? ; } } self . seen_value = true ; value . serialize (ValueSerializer :: with_style (dst , self . style)) ? ; if self . style . multiline_array { dst . val_sep () ? ; } Ok (()) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { let dst = self . table . body_mut () ; if self . style . multiline_array && self . seen_value { dst . newline () ? ; } dst . close_array () ? ; dst . newline () ? ; self . buf . push (self . table) ; Ok (self . buf) } }
    };
}

impl_267!();