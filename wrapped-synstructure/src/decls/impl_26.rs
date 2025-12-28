macro_rules! deps {
    () => {
        MacroResult!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : MacroResult > MacroResult for Result < T > { fn into_result (self) -> Result < TokenStream > { match self { Ok (v) => v . into_result () , Err (err) => Err (err) , } } }
    };
}

impl_26!()