macro_rules! with_vars_unset {
    () => {
        # [doc = " Unsets environment variables for the duration of the closure."] # [doc = ""] # [doc = " The previous values are restored when the closure completes or panics, before unwinding the"] # [doc = " panic."] # [doc = ""] # [doc = " This is a shorthand and identical to the following:"] # [doc = " ```rust"] # [doc = " temp_env::with_vars("] # [doc = "     ["] # [doc = "         (\"FIRST_VAR\", None::<&str>),"] # [doc = "         (\"SECOND_VAR\", None::<&str>),"] # [doc = "     ],"] # [doc = "     || {"] # [doc = "         // Run some code where `FIRST_VAR` and `SECOND_VAR` are unset (even if"] # [doc = "         // they were set before)"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] pub fn with_vars_unset < K , F , R > (keys : impl AsRef < [K] > , closure : F) -> R where K : AsRef < OsStr > + Clone + Eq + Hash , F : FnOnce () -> R , { let kvs = keys . as_ref () . iter () . map (| key | (key , None :: < & str >)) . collect :: < Vec < _ > > () ; with_vars (kvs , closure) }
    };
}

with_vars_unset!();