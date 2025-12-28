macro_rules! deps {
    () => {
        FieldSet!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl fmt :: Display for FieldSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . names . iter () . map (display)) . finish () } }
    };
}

impl_177!();