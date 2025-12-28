macro_rules! deps {
    () => {
        DynFilterFn!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < S , F , R > Clone for DynFilterFn < S , F , R > where F : Clone , R : Clone , { fn clone (& self) -> Self { Self { enabled : self . enabled . clone () , register_callsite : self . register_callsite . clone () , max_level_hint : self . max_level_hint , _s : PhantomData , } } }
    };
}

impl_68!()