macro_rules! format_timestamp_with_decimals {
    () => {
        fn format_timestamp_with_decimals (ansi : bool , elapsed : Duration , w : & mut impl Write ,) -> std :: fmt :: Result { let secs = elapsed . as_secs_f64 () ; let (n , unit) = if secs < 0.001 { (secs * 1_000_000.0 , "μs") } else if secs < 1.0 { (secs * 1_000.0 , "ms") } else { (secs , "s ") } ; let timestamp = format ! (" {n:.2}") ; write_style_timestamp (ansi , timestamp , unit , w) }
    };
}

format_timestamp_with_decimals!()