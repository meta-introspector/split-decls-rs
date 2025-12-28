macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary")] # [cfg_attr (docsrs , doc (cfg (feature = "arbitrary")))] impl < 'a > arbitrary :: Arbitrary < 'a > for SmolStr { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> Result < Self , arbitrary :: Error > { let s = < & str > :: arbitrary (u) ? ; Ok (SmolStr :: new (s)) } }
    };
}

impl_66!();