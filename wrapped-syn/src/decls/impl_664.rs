macro_rules! deps {
    () => {
        Punctuated!();
        Pair!();
        End!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Clone for Pair < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { match self { Pair :: Punctuated (t , p) => Pair :: Punctuated (t . clone () , p . clone ()) , Pair :: End (t) => Pair :: End (t . clone ()) , } } }
    };
}

impl_664!();