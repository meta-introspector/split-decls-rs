macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! TokenText {
    () => {
        deps!();
        pub struct TokenText < 'a > (pub (crate) Repr < 'a >) ;
    };
}

TokenText!()