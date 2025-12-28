macro_rules! deps {
    () => {
        SelfBounds!();
        Trait!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [allow (clippy :: needless_lifetimes)] impl < 'a > SelfBounds < 'a > { const SIZED : Self = Self :: All (& [Trait :: Sized]) ; }
    };
}

impl_59!()