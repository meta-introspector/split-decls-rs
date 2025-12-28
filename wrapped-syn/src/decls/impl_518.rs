macro_rules! deps {
    () => {
        ParseStream!();
        Result!();
        Parse!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl < T : Parse > Parse for Box < T > { fn parse (input : ParseStream) -> Result < Self > { input . parse () . map (Box :: new) } }
    };
}

impl_518!()