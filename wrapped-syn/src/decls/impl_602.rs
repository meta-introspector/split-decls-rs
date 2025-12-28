macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_602 {
    () => {
        deps!();
        # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Clone for Punctuated < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { Punctuated { inner : self . inner . clone () , last : self . last . clone () , } } fn clone_from (& mut self , other : & Self) { self . inner . clone_from (& other . inner) ; self . last . clone_from (& other . last) ; } }
    };
}

impl_602!()