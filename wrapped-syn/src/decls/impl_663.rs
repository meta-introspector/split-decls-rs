macro_rules! deps {
    () => {
        Punctuated!();
        Pair!();
        End!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Pair < & T , & P > { pub fn cloned (self) -> Pair < T , P > where T : Clone , P : Clone , { match self { Pair :: Punctuated (t , p) => Pair :: Punctuated (t . clone () , p . clone ()) , Pair :: End (t) => Pair :: End (t . clone ()) , } } }
    };
}

impl_663!();