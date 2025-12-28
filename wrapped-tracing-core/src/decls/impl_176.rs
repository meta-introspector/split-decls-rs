macro_rules! deps {
    () => {
        FieldSet!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl fmt :: Debug for FieldSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FieldSet") . field ("names" , & self . names) . field ("callsite" , & self . callsite) . finish () } }
    };
}

impl_176!();