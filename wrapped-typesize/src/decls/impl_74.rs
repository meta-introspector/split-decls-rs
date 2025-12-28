macro_rules! deps {
    () => {
        SizableArc!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T , SC : ShouldCountInner > From < Arc < T > > for SizableArc < T , SC > { fn from (value : Arc < T >) -> Self { SizableArc (value , PhantomData) } }
    };
}

impl_74!();