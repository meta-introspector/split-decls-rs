macro_rules! deps {
    () => {
        Parser!();
        Parse!();
        ParseNestedMeta!();
        Result!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl MetaList { # [doc = " See [`Attribute::parse_args`]."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn parse_args < T : Parse > (& self) -> Result < T > { self . parse_args_with (T :: parse) } # [doc = " See [`Attribute::parse_args_with`]."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn parse_args_with < F : Parser > (& self , parser : F) -> Result < F :: Output > { let scope = self . delimiter . span () . close () ; crate :: parse :: parse_scoped (parser , scope , self . tokens . clone ()) } # [doc = " See [`Attribute::parse_nested_meta`]."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn parse_nested_meta (& self , logic : impl FnMut (ParseNestedMeta) -> Result < () > ,) -> Result < () > { self . parse_args_with (meta :: parser (logic)) } }
    };
}

impl_76!();