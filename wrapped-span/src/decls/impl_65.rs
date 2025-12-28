macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SyntaxContext { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { fmt :: Display :: fmt (self , f) } else { f . debug_tuple ("SyntaxContext") . field (& self . 0) . finish () } } }
    };
}

impl_65!();