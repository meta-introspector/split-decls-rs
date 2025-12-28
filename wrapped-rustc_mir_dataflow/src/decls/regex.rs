macro_rules! regex {
    () => {
        macro_rules ! regex { ($ re : literal $ (,) ?) => { { static RE : OnceLock < regex :: Regex > = OnceLock :: new () ; RE . get_or_init (|| Regex :: new ($ re) . unwrap ()) } } ; }
    };
}

regex!();