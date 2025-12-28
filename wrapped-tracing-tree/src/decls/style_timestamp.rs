macro_rules! style_timestamp {
    () => {
        fn style_timestamp (ansi : bool , higher_precision : bool , elapsed : Duration , w : & mut impl Write ,) -> std :: fmt :: Result { if higher_precision { format_timestamp_with_decimals (ansi , elapsed , w) } else { format_timestamp (ansi , elapsed , w) } }
    };
}

style_timestamp!();