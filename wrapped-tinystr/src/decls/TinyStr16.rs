macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! TinyStr16 {
    () => {
        deps!();
        # [doc = " These are temporary compatability reexports that will be removed"] # [doc = " in a future version."] pub type TinyStr16 = TinyAsciiStr < 16 > ;
    };
}

TinyStr16!()