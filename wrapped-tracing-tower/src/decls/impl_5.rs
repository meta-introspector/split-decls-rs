macro_rules! deps {
    () => {
        GetSpan!();
        Service!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < S , R , G > Service < S , R , G > where S : tower_service :: Service < R > , G : GetSpan < R > + Clone , { pub fn new (inner : S , get_span : G) -> Self { Service { get_span , inner , _p : PhantomData , } } }
    };
}

impl_5!();