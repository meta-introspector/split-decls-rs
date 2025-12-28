macro_rules! BANNED_FIELD_NAMES {
    () => {
        const BANNED_FIELD_NAMES : & [& str] = & ["from" , "new"] ;
    };
}

BANNED_FIELD_NAMES!();