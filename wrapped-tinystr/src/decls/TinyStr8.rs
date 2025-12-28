macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! TinyStr8 {
    () => {
        deps!();
        # [doc = " These are temporary compatability reexports that will be removed"] # [doc = " in a future version."] pub type TinyStr8 = TinyAsciiStr < 8 > ;
    };
}

TinyStr8!()