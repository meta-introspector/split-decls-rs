macro_rules! deps {
    () => {
        FormatTime!();
        Uptime!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl FormatTime for Uptime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { let e = self . epoch . elapsed () ; write ! (w , "{:4}.{:06}s" , e . as_secs () , e . subsec_micros ()) } fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result { style_timestamp (ansi , self . higher_precision , elapsed , w) } }
    };
}

impl_30!()