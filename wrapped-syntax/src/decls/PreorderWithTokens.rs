macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! PreorderWithTokens {
    () => {
        deps!();
        pub type PreorderWithTokens = rowan :: api :: PreorderWithTokens < RustLanguage > ;
    };
}

PreorderWithTokens!();