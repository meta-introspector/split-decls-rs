macro_rules! deps {
    () => {
        QueryStackDeferred!();
        QueryStackFrameExtra!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'tcx > QueryStackDeferred < 'tcx > { pub fn new < C : Copy + DynSync + DynSend + 'tcx > (context : C , extract : fn (C) -> QueryStackFrameExtra ,) -> Self { let extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend + 'tcx > = Arc :: new (move | | extract (context)) ; Self { _dummy : PhantomData , extract : unsafe { transmute (extract) } } } pub fn extract (& self) -> QueryStackFrameExtra { (self . extract) () } }
    };
}

impl_209!();