macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        # [allow (missing_docs)] impl < T > UnionField < T > { # [inline] pub const fn new () -> Self { Self (:: core :: marker :: PhantomData) } # [inline] pub unsafe fn as_ref (& self) -> & T { :: core :: mem :: transmute (self) } # [inline] pub unsafe fn as_mut (& mut self) -> & mut T { :: core :: mem :: transmute (self) } }
    };
}

impl_325!()