macro_rules! deps {
    () => {
        TraitBound!();
        FieldBounds!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'a > FieldBounds < 'a > { const ALL_SELF : FieldBounds < 'a > = FieldBounds :: All (& [TraitBound :: Slf]) ; const TRAILING_SELF : FieldBounds < 'a > = FieldBounds :: Trailing (& [TraitBound :: Slf]) ; }
    };
}

impl_112!();