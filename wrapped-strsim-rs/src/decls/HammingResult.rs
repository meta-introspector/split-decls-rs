macro_rules! deps {
    () => {
        StrSimError!();
    };
}

macro_rules! HammingResult {
    () => {
        deps!();
        pub type HammingResult = Result < usize , StrSimError > ;
    };
}

HammingResult!()