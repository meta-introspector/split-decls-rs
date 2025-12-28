macro_rules! is_below_max_level {
    () => {
        fn is_below_max_level (hint : & Option < LevelFilter > , metadata : & Metadata < '_ >) -> bool { hint . as_ref () . map (| hint | metadata . level () <= hint) . unwrap_or (true) }
    };
}

is_below_max_level!();