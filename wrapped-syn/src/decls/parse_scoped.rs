macro_rules! deps {
    () => {
        Result!();
        Parser!();
    };
}

macro_rules! parse_scoped {
    () => {
        deps!();
        pub (crate) fn parse_scoped < F : Parser > (f : F , scope : Span , tokens : TokenStream) -> Result < F :: Output > { f . __parse_scoped (scope , tokens) }
    };
}

parse_scoped!()