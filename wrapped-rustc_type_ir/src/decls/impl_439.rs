macro_rules! deps {
    () => {
        IntVarValue!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl IntVarValue { pub fn is_known (self) -> bool { match self { IntVarValue :: IntType (_) | IntVarValue :: UintType (_) => true , IntVarValue :: Unknown => false , } } pub fn is_unknown (self) -> bool { ! self . is_known () } }
    };
}

impl_439!();