macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T , P > PartialEq for Punctuated < T , P > where T : PartialEq , P : PartialEq , { fn eq (& self , other : & Self) -> bool { let Punctuated { inner , last } = self ; * inner == other . inner && * last == other . last } }
    };
}

impl_604!()