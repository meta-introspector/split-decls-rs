macro_rules! deps {
    () => {
        ExpectedFields!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl fmt :: Display for ExpectedFields { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "fields ") ? ; let entries = self . fields . iter () . map (| (k , v) | (field :: display (k) , field :: display (v))) ; f . debug_map () . entries (entries) . finish () } }
    };
}

impl_38!();