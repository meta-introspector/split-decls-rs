macro_rules! deps {
    () => {
        FloatVarValue!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl FloatVarValue { pub fn is_known (self) -> bool { match self { FloatVarValue :: Known (_) => true , FloatVarValue :: Unknown => false , } } pub fn is_unknown (self) -> bool { ! self . is_known () } }
    };
}

impl_441!();