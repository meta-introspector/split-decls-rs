macro_rules! deps {
    () => {
        MacroResult!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl MacroResult for TokenStream { fn into_result (self) -> Result < TokenStream > { Ok (self) } }
    };
}

impl_22!()