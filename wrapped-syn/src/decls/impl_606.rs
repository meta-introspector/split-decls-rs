macro_rules! deps {
    () => {
        Result!();
        Punctuated!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T : Debug , P : Debug > Debug for Punctuated < T , P > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut list = f . debug_list () ; for (t , p) in & self . inner { list . entry (t) ; list . entry (p) ; } if let Some (last) = & self . last { list . entry (last) ; } list . finish () } }
    };
}

impl_606!()