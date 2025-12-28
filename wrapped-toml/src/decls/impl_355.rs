macro_rules! deps {
    () => {
        Style!();
        Error!();
        SerializeTable!();
        SerializeStructVariant!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < 'd > SerializeStructVariant < 'd > { pub (crate) fn struct_ (dst : & 'd mut String , variant : & 'static str , _len : usize , style : Style ,) -> Result < Self , Error > { dst . open_inline_table () ? ; dst . space () ? ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; Ok (Self { inner : SerializeTable :: map (dst , style) ? , }) } }
    };
}

impl_355!();