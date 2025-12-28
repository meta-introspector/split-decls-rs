macro_rules! deps {
    () => {
        FormatTime!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [doc = " Default do-nothing time formatter."] impl FormatTime for () { fn format_time (& self , _w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { Ok (()) } fn style_timestamp (& self , _ansi : bool , _elapsed : Duration , _w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { Ok (()) } }
    };
}

impl_22!()