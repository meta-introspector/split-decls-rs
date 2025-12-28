macro_rules! deps {
    () => {
        FieldBounds!();
        TraitBound!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > FieldBounds < 'a > { const ALL_SELF : FieldBounds < 'a > = FieldBounds :: All (& [TraitBound :: Slf]) ; const TRAILING_SELF : FieldBounds < 'a > = FieldBounds :: Trailing (& [TraitBound :: Slf]) ; }
    };
}

impl_57!()