macro_rules! deps {
    () => {
        Buffer!();
        Table!();
        SerializeDocumentTupleVariant!();
        Style!();
        Error!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < 'd > SerializeDocumentTupleVariant < 'd > { pub (crate) fn tuple (buf : & 'd mut Buffer , mut table : Table , variant : & 'static str , _len : usize , style : Style ,) -> Result < Self , Error > { let dst = table . body_mut () ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; dst . open_array () ? ; Ok (Self { buf , table , seen_value : false , style , }) } }
    };
}

impl_266!()