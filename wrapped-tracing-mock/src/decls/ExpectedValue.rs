macro_rules! ExpectedValue {
    () => {
        # [derive (Debug)] pub (crate) enum ExpectedValue { F64 (f64) , I64 (i64) , U64 (u64) , Bool (bool) , Str (String) , Debug (String) , Any , }
    };
}

ExpectedValue!();