macro_rules! deps {
    () => {
        Style!();
        Error!();
        SerializeTupleVariant!();
        SerializeValueArray!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'd > SerializeTupleVariant < 'd > { pub (crate) fn tuple (dst : & 'd mut String , variant : & 'static str , len : usize , style : Style ,) -> Result < Self , Error > { dst . open_inline_table () ? ; dst . space () ? ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; Ok (Self { inner : SerializeValueArray :: seq (dst , style , Some (len)) ? , }) } }
    };
}

impl_332!()