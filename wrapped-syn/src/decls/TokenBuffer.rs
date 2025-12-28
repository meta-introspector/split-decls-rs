macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! TokenBuffer {
    () => {
        deps!();
        # [doc = " A buffer that can be efficiently traversed multiple times, unlike"] # [doc = " `TokenStream` which requires a deep copy in order to traverse more than"] # [doc = " once."] pub struct TokenBuffer { entries : Box < [Entry] > , }
    };
}

TokenBuffer!();