macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (not (feature = "alloc"))] impl < 'computed > Clone for Label < 'computed > { fn clone (& self) -> Self { Label { value_computed : self . value_computed , backing_field_static : self . backing_field_static , tag : self . tag . clone () , _marker : PhantomData , } } }
    };
}

impl_16!()