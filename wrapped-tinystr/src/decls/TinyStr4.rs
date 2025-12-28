macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! TinyStr4 {
    () => {
        deps!();
        # [doc = " These are temporary compatability reexports that will be removed"] # [doc = " in a future version."] pub type TinyStr4 = TinyAsciiStr < 4 > ;
    };
}

TinyStr4!();