macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A type alias that maps to `std::result::Result<T, ucd_trie::Error>`."] pub type Result < T > = result :: Result < T , Error > ;
    };
}

Result!()