macro_rules! deps {
    () => {
        TraitBound!();
    };
}

macro_rules! FieldBounds {
    () => {
        deps!();
        enum FieldBounds < 'a > { None , All (& 'a [TraitBound]) , Trailing (& 'a [TraitBound]) , Explicit (Vec < WherePredicate >) , }
    };
}

FieldBounds!();