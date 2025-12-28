macro_rules! deps {
    () => {
        FormatTime!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < F > FormatTime for & F where F : FormatTime , { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { F :: format_time (self , w) } fn style_timestamp (& self , ansi : bool , duration : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { F :: style_timestamp (self , ansi , duration , w) } }
    };
}

impl_35!();