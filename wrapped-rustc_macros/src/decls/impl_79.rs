macro_rules! deps {
    () => {
        ExtensionAttr!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Parse for ExtensionAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let vis = input . parse () ? ; let _ : Token ! [trait] = input . parse () ? ; let trait_ = input . parse () ? ; Ok (ExtensionAttr { vis , trait_ }) } }
    };
}

impl_79!()