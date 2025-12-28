macro_rules! deps {
    () => {
        Service!();
        GetSpan!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < S , R , G > Clone for Service < S , R , G > where S : tower_service :: Service < R > + Clone , G : GetSpan < R > + Clone , { fn clone (& self) -> Self { Service { get_span : self . get_span . clone () , inner : self . inner . clone () , _p : PhantomData , } } }
    };
}

impl_4!()