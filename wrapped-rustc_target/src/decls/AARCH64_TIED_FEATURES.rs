macro_rules! AARCH64_TIED_FEATURES {
    () => {
        const AARCH64_TIED_FEATURES : & [& [& str]] = & [& ["paca" , "pacg"] ,] ;
    };
}

AARCH64_TIED_FEATURES!();