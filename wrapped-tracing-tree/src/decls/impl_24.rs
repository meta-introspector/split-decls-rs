macro_rules! deps {
    () => {
        FormatTime!();
        UtcDateTime!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "time")] impl FormatTime for UtcDateTime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { let time = time :: OffsetDateTime :: now_utc () ; write ! (w , "{} {}" , time . date () , time . time ()) } fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { style_timestamp (ansi , self . higher_precision , elapsed , w) } }
    };
}

impl_24!();