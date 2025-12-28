macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for Table { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let children = self . get_values () ; for (key_path , value) in children { crate :: encode :: encode_key_path_ref (& key_path , f , None , DEFAULT_KEY_DECOR) ? ; write ! (f , "=") ? ; crate :: encode :: encode_value (value , f , None , DEFAULT_VALUE_DECOR) ? ; writeln ! (f) ? ; } Ok (()) } }
    };
}

impl_229!();