macro_rules! deps {
    () => {
        SizableRc!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T , SC : ShouldCountInner > From < Rc < T > > for SizableRc < T , SC > { fn from (value : Rc < T >) -> Self { SizableRc (value , PhantomData) } }
    };
}

impl_81!()